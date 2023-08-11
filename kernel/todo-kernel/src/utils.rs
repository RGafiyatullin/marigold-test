use tezos_smart_rollup_debug::debug_msg;

use crate::kernel::Kernel;

pub fn report_error(mut kernel: impl Kernel, slogan: &str, mut error: &dyn std::error::Error) {
    let h = kernel.host();
    debug_msg!(h, "{}", slogan);
    loop {
        debug_msg!(h, " * {}", error);
        let Some(next) = error.source() else { break };
        error = next;
    }
}
