use std::usize;

use crate::cpu::riscv::exception::Exception;
use crate::cpu::riscv::riscv32::decode::*;
use crate::cpu::riscv::riscv32::riscv32::RiscV32;

impl RiscV32 {
    pub fn execute_op(&mut self, inst: u32) {
        let funt3 = extract_bits(inst, 12, 14);
        let funt7 = extract_bits(inst, 25, 31);
        let rd = extract_bits(inst, 7, 11) as usize;
        let rs1 = extract_bits(inst, 15, 19) as usize;
        let rs2 = extract_bits(inst, 20, 24) as usize;

        if rd == 0 {
            return;
        }

        match funt3 {
            0x0 => match funt7 {
                0x00 => self.reg[rd] = self.reg[rs1].wrapping_add(self.reg[rs2]),
                0x20 => self.reg[rd] = self.reg[rs1].wrapping_sub(self.reg[rs2]),
                _ => self.except = Some(Exception::IllegalInstruction),
            },
            0x1 => self.reg[rd] = self.reg[rs1].wrapping_shl(self.reg[rs2] & 0x1F),
            0x2 => {
                self.reg[rd] = if (self.reg[rs1] as i32) < (self.reg[rs2] as i32) {
                    1
                } else {
                    0
                }
            }
            0x5 => match funt7 {
                0x00 => self.reg[rd] = self.reg[rs1].wrapping_shr(self.reg[rs2] & 0x1F),
                0x20 => {
                    self.reg[rd] = (self.reg[rs1] as i32).wrapping_shr(self.reg[rs2] & 0x1F) as u32
                }
                _ => self.except = Some(Exception::IllegalInstruction),
            },
            0x3 => self.reg[rd] = if self.reg[rs1] < self.reg[rs2] { 1 } else { 0 },
            0x4 => self.reg[rd] = self.reg[rs1] ^ self.reg[rs2],
            0x6 => self.reg[rd] = self.reg[rs1] | self.reg[rs2],
            0x7 => self.reg[rd] = self.reg[rs1] & self.reg[rs2],
            _ => self.except = Some(Exception::IllegalInstruction),
        }
    }
}
