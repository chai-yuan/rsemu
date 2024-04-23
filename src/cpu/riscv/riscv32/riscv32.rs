use crate::bus::bus::Bus;
use crate::cpu::riscv::exception::Exception;
use crate::cpu::CPU;

pub struct RiscV32 {
    // 处理器寄存器
    pub reg: [u32; 32],
    pub pc: u32,
    // 处理器其他状态
    pub except: Option<Exception>,
}

impl RiscV32 {
    pub fn new() -> Self {
        Self {
            reg: [0; 32],
            pc: 0,
            except: None,
        }
    }
}

impl CPU for RiscV32 {
    fn step(&mut self, bus: &mut Bus) {
        // 取指令
        let inst = bus.read(self.pc as u64, 4) as u32;
        // 执行
        match inst & 0x7f {
            0b0110011 => self.execute_op(inst),
            _ => self.except = Some(Exception::IllegalInstruction),
        }
    }
}
