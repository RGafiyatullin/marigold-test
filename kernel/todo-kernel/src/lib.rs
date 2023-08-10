use tezos_smart_rollup::kernel_entry;
use tezos_smart_rollup::prelude::Runtime;
use tezos_smart_rollup_debug::debug_msg;

pub mod message;

mod blake2;
mod data;
mod kernel;

use kernel::{Kernel, KernelReadInputExt, ReadInputError};

pub fn entry<Host: Runtime>(host: &mut Host) {
    debug_msg!(host, "ENTER");
    let kernel = kernel::init(host);
    if let Err(reason) = run(kernel) {
        debug_msg!(host, "{}", reason);
        let mut e: &dyn std::error::Error = &reason;
        while let Some(source) = e.source() {
            debug_msg!(host, "- {}", source);
            e = source;
        }
    }
    debug_msg!(host, "LEAVE");
}

kernel_entry!(entry);

#[derive(Debug, thiserror::Error)]
enum KernelRunError {
    // #[error("Not Implemented")]
    // NotImplemented,
    #[error("Error reading input")]
    ReadInputError(#[source] ReadInputError),
}

fn run(mut kernel: impl Kernel) -> Result<(), KernelRunError> {
    debug_msg!(kernel.host(), "run ENTER");
    loop {
        let Some(message) = kernel.next_message().map_err(KernelRunError::ReadInputError)? else {
            debug_msg!(kernel.host(), "Inbox empty, Ciao!");
            break Ok(())
        };

        debug_msg!(kernel.host(), "Message: {:?}", message);
    }
}
