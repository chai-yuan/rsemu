use crate::cpu::riscv::exception::Exception;
use crate::cpu::riscv::riscv32::decode::*;
use crate::cpu::riscv::riscv32::riscv32::RiscV32;

impl RiscV32 {
    pub fn execute_op_imm(&mut self, inst: u32) {
        let funct3 = extract_bits(inst, 12, 14);
        let funct7 = extract_bits(inst, 25, 31);
        let rd = extract_bits(inst, 7, 11) as usize;
        let rs1 = extract_bits(inst, 15, 19) as usize;
        let imm = decode_sign_imm!(inst, 20, 31) as u32;

        if rd == 0 {
            return;
        }

        match funct3 {
            0b000 => {
                self.reg[rd] = self.reg[rs1].wrapping_add(imm);
            }
            0b010 => {
                self.reg[rd] = if (self.reg[rs1] as i32) < imm as i32 {
                    1
                } else {
                    0
                };
            }
            0b011 => {
                self.reg[rd] = if (self.reg[rs1] as u32) < imm as u32 {
                    1
                } else {
                    0
                };
            }
            0b100 => {
                self.reg[rd] = self.reg[rs1] ^ imm;
            }
            0b110 => {
                self.reg[rd] = self.reg[rs1] | imm;
            }
            0b111 => {
                self.reg[rd] = self.reg[rs1] & imm;
            }
            0b001 => {
                if funct7 == 0b0000000 {
                    self.reg[rd] = self.reg[rs1] << (imm & 0x1F);
                } else {
                    self.except = Some(Exception::IllegalInstruction);
                }
            }
            0b101 => {
                if funct7 == 0b0000000 {
                    self.reg[rd] = self.reg[rs1] >> (imm & 0x1F);
                } else if funct7 == 0b0100000 {
                    self.reg[rd] = ((self.reg[rs1] as i32) >> (imm & 0x1F)) as u32;
                } else {
                    self.except = Some(Exception::IllegalInstruction);
                }
            }
            _ => {
                self.except = Some(Exception::IllegalInstruction);
            }
        }
    }
}
