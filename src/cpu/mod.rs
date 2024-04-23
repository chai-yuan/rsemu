use crate::bus::bus::Bus;

pub mod riscv;

pub trait CPU {
    // 执行一条指令
    fn step(&mut self, bus: &mut Bus);
}
