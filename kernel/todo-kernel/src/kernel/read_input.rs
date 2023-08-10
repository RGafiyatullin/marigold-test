use tezos_smart_rollup::host::RuntimeError;
use tezos_smart_rollup_debug::{debug_msg, Runtime};

use crate::message::TodoMessage;

use super::Kernel;

#[derive(Debug, thiserror::Error)]
pub enum ReadInputError {
    #[error("Runtime Error")]
    RuntimeError(#[source] RuntimeError),

    #[error("Decode Error")]
    DecodeError(#[source] crate::message::DecodeError),

    #[error("Not Implemented")]
    NotImplemented,
}

pub trait KernelReadInputExt: Kernel {
    fn next_message(&mut self) -> Result<Option<TodoMessage>, ReadInputError> {
        let Some(message) = self.host().read_input().map_err(ReadInputError::RuntimeError)? else {
            return Ok(None)
        };
        match message.as_ref() {
            [0x00, internal @ ..] => unimplemented!("Internal: {:?}", internal),
            [0x01, external @ ..] => {
                let todo_message =
                    TodoMessage::decode(external).map_err(ReadInputError::DecodeError)?;
                debug_msg!(self.host(), "todo-message: {:?}", todo_message);
                Err(ReadInputError::NotImplemented)
            },
            [..] => unimplemented!("Unsupported message-type"),
        }
    }
}

impl<K> KernelReadInputExt for K where K: Kernel {}
