use crate::bus::bus::Bus;

#[macro_use]
pub mod decode;
pub mod riscv32;

pub trait CPU {
    // 执行一条指令
    fn execute(&mut self, bus: &mut Bus);
}
