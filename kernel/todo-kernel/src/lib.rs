use tezos_smart_rollup::kernel_entry;
use tezos_smart_rollup::prelude::Runtime;
use tezos_smart_rollup_debug::debug_msg;

pub mod message;

mod data;
mod dispatch;
mod kernel;
mod signing;
mod utils;

use kernel::{Kernel, KernelReadInputExt, ReadInputError};

pub fn entry<Host: Runtime>(host: &mut Host) {
    debug_msg!(host, "ENTER");
    {
        let mut kernel = kernel::init(host);
        if let Err(reason) = run(&mut kernel) {
            utils::report_error(&mut kernel, "Kernel Failure", &reason);
        }
    }
    debug_msg!(host, "LEAVE");
}

kernel_entry!(entry);

#[derive(Debug, thiserror::Error)]
enum KernelFailure {
    #[error("Error reading input")]
    ReadInputError(#[source] ReadInputError),
}

fn run(mut kernel: impl Kernel) -> Result<(), KernelFailure> {
    debug_msg!(kernel.host(), "run ENTER");
    loop {
        // TODO: do not bail out if just a single message is corrupted.
        let Some(message) = kernel.next_message().map_err(KernelFailure::ReadInputError)? else {
            debug_msg!(kernel.host(), "Inbox empty, Bye!");
            break Ok(())
        };

        debug_msg!(kernel.host(), "Message: {:?}", message);

        if let Err(reason) = dispatch::process_inbound_message(&mut kernel, message) {
            utils::report_error(&mut kernel, "Error processing message", &reason);
        }
    }
}
