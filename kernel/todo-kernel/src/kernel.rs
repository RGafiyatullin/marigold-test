use tezos_smart_rollup::prelude::Runtime;

mod read_input;
pub use read_input::{KernelReadInputExt, ReadInputError};

pub trait Kernel {
    type Host: Runtime;
    fn host(&mut self) -> &mut Self::Host;
}

struct K<'a, H>(&'a mut H);

pub fn init<H>(host: &mut H) -> impl Kernel + '_
where
    H: Runtime,
{
    K(host)
}

impl<'a, H> Kernel for K<'a, H>
where
    H: Runtime,
{
    type Host = H;
    fn host(&mut self) -> &mut Self::Host {
        self.0
    }
}

impl<'a, K> Kernel for &'a mut K
where
    K: Kernel,
{
    type Host = <K as Kernel>::Host;
    fn host(&mut self) -> &mut Self::Host {
        <K as Kernel>::host(*self)
    }
}
