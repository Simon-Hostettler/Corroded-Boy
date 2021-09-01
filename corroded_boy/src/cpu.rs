use crate::memory::Memory;
use crate::register::Flags::{FC, FH, FN, FZ};
use crate::register::RegisterFile;
use crate::register::Registers16b::{AF, BC, DE, HL};
use crate::register::Registers8b;
use crate::register::Registers8b::{A, B, C, D, E, F, H, L};

pub struct CPU {
    pub reg: RegisterFile,
    pub mem: Memory,
    pub halted: bool,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            reg: RegisterFile::new(),
            mem: Memory::new(),
            halted: false,
        }
    }

    fn fetch_byte(&self) -> u8 {
        0 // placeholder
    }

    fn fetch_word(&self) -> u16 {
        0 // placeholder
    }

    fn execute(&mut self) {
        let operation = self.fetch_byte();
        match operation {
            0x00 => {} //nop
            0x01 => {
                let d16 = self.fetch_word();
                self.reg.write_16b(BC, d16);
            }
            0x02 => {
                self.mem.wb(self.reg.read_16b(BC), self.reg.a);
            }
            0x03 => {}
            0x04 => {}
            0x05 => {}
            0x06 => {}
            0x07 => {}
            0x08 => {}
            0x09 => {}
            0x0A => {}
            0x0B => {}
            0x0C => {}
            0x0D => {}
            0x0E => {}
            0x0F => {}
            0x10 => {}
            0x11 => {}
            0x12 => {}
            0x13 => {}
            0x14 => {}
            0x15 => {}
            0x16 => {}
            0x17 => {}
            0x18 => {}
            0x19 => {}
            0x1A => {}
            0x1B => {}
            0x1C => {}
            0x1D => {}
            0x1E => {}
            0x1F => {}
            0x20 => {}
            0x21 => {}
            0x22 => {}
            0x23 => {}
            0x24 => {}
            0x25 => {}
            0x26 => {}
            0x27 => {}
            0x28 => {}
            0x29 => {}
            0x2A => {}
            0x2B => {}
            0x2C => {}
            0x2D => {}
            0x2E => {}
            0x2F => {}
            0x30 => {}
            0x31 => {}
            0x32 => {}
            0x33 => {}
            0x34 => {}
            0x35 => {}
            0x36 => {}
            0x37 => {}
            0x38 => {}
            0x39 => {}
            0x3A => {}
            0x3B => {}
            0x3C => {}
            0x3D => {}
            0x3E => {}
            0x3F => {}
            0x40 => self.reg.b = self.reg.b,
            0x41 => self.reg.b = self.reg.c,
            0x42 => self.reg.b = self.reg.d,
            0x43 => self.reg.b = self.reg.e,
            0x44 => self.reg.b = self.reg.h,
            0x45 => self.reg.b = self.reg.l,
            0x46 => self.reg.b = self.mem.rb(self.reg.read_16b(HL)),
            0x47 => self.reg.b = self.reg.a,
            0x48 => self.reg.c = self.reg.b,
            0x49 => self.reg.c = self.reg.c,
            0x4B => self.reg.c = self.reg.d,
            0x4C => self.reg.c = self.reg.e,
            0x4D => self.reg.c = self.reg.h,
            0x4A => self.reg.c = self.reg.l,
            0x4E => self.reg.c = self.mem.rb(self.reg.read_16b(HL)),
            0x4F => self.reg.c = self.reg.a,
            0x50 => self.reg.d = self.reg.b,
            0x51 => self.reg.d = self.reg.c,
            0x52 => self.reg.d = self.reg.d,
            0x53 => self.reg.d = self.reg.e,
            0x54 => self.reg.d = self.reg.h,
            0x55 => self.reg.d = self.reg.l,
            0x56 => self.reg.d = self.mem.rb(self.reg.read_16b(HL)),
            0x57 => self.reg.d = self.reg.a,
            0x58 => self.reg.e = self.reg.b,
            0x59 => self.reg.e = self.reg.c,
            0x5A => self.reg.e = self.reg.d,
            0x5B => self.reg.e = self.reg.e,
            0x5C => self.reg.e = self.reg.h,
            0x5D => self.reg.e = self.reg.l,
            0x5E => self.reg.e = self.mem.rb(self.reg.read_16b(HL)),
            0x5F => self.reg.e = self.reg.a,
            0x60 => self.reg.h = self.reg.b,
            0x61 => self.reg.h = self.reg.c,
            0x62 => self.reg.h = self.reg.d,
            0x63 => self.reg.h = self.reg.e,
            0x64 => self.reg.h = self.reg.h,
            0x65 => self.reg.h = self.reg.l,
            0x66 => self.reg.h = self.mem.rb(self.reg.read_16b(HL)),
            0x67 => self.reg.h = self.reg.a,
            0x68 => self.reg.l = self.reg.b,
            0x69 => self.reg.l = self.reg.c,
            0x6A => self.reg.l = self.reg.d,
            0x6B => self.reg.l = self.reg.e,
            0x6C => self.reg.l = self.reg.h,
            0x6D => self.reg.l = self.reg.l,
            0x6E => self.reg.l = self.mem.rb(self.reg.read_16b(HL)),
            0x6F => self.reg.l = self.reg.a,
            0x70 => self.mem.wb(self.reg.read_16b(HL), self.reg.b),
            0x71 => self.mem.wb(self.reg.read_16b(HL), self.reg.c),
            0x72 => self.mem.wb(self.reg.read_16b(HL), self.reg.d),
            0x73 => self.mem.wb(self.reg.read_16b(HL), self.reg.e),
            0x74 => self.mem.wb(self.reg.read_16b(HL), self.reg.h),
            0x75 => self.mem.wb(self.reg.read_16b(HL), self.reg.l),
            0x76 => self.halted = true,
            0x77 => self.mem.wb(self.reg.read_16b(HL), self.reg.a),
            0x78 => self.reg.a = self.reg.b,
            0x79 => self.reg.a = self.reg.c,
            0x7A => self.reg.a = self.reg.d,
            0x7B => self.reg.a = self.reg.e,
            0x7C => self.reg.a = self.reg.h,
            0x7D => self.reg.a = self.reg.l,
            0x7E => self.reg.a = self.mem.rb(self.reg.read_16b(HL)),
            0x7F => self.reg.a = self.reg.a,
            0x80 => self.alu_add(B),
            0x81 => self.alu_add(C),
            0x82 => self.alu_add(D),
            0x83 => self.alu_add(E),
            0x84 => self.alu_add(H),
            0x85 => self.alu_add(L),
            0x86 => {}
            0x87 => {}
            0x88 => {}
            0x89 => {}
            0x8A => {}
            0x8B => {}
            0x8C => {}
            0x8D => {}
            0x8E => {}
            0x8F => {}
            0x90 => {}
            0x91 => {}
            0x92 => {}
            0x93 => {}
            0x94 => {}
            0x95 => {}
            0x96 => {}
            0x97 => {}
            0x98 => {}
            0x99 => {}
            0x9A => {}
            0x9B => {}
            0x9C => {}
            0x9D => {}
            0x9E => {}
            0x9F => {}
            0xA0 => {}
            0xA1 => {}
            0xA2 => {}
            0xA3 => {}
            0xA4 => {}
            0xA5 => {}
            0xA6 => {}
            0xA7 => {}
            0xA8 => {}
            0xA9 => {}
            0xAA => {}
            0xAB => {}
            0xAC => {}
            0xAD => {}
            0xAE => {}
            0xAF => {}
            0xB0 => {}
            0xB1 => {}
            0xB2 => {}
            0xB3 => {}
            0xB4 => {}
            0xB5 => {}
            0xB6 => {}
            0xB7 => {}
            0xB8 => {}
            0xB9 => {}
            0xBA => {}
            0xBB => {}
            0xBC => {}
            0xBD => {}
            0xBE => {}
            0xBF => {}
            0xC0 => {}
            0xC1 => {}
            0xC2 => {}
            0xC3 => {}
            0xC4 => {}
            0xC5 => {}
            0xC6 => {}
            0xC7 => {}
            0xC8 => {}
            0xC9 => {}
            0xCA => {}
            0xCB => {}
            0xCC => {}
            0xCD => {}
            0xCE => {}
            0xCF => {}
            0xD0 => {}
            0xD1 => {}
            0xD2 => {}
            0xD3 => {}
            0xD4 => {}
            0xD5 => {}
            0xD6 => {}
            0xD7 => {}
            0xD8 => {}
            0xD9 => {}
            0xDA => {}
            0xDB => {}
            0xDC => {}
            0xDD => {}
            0xDE => {}
            0xDF => {}
            0xE0 => {}
            0xE1 => {}
            0xE2 => {}
            0xE3 => {}
            0xE4 => {}
            0xE5 => {}
            0xE6 => {}
            0xE7 => {}
            0xE8 => {}
            0xE9 => {}
            0xEA => {}
            0xEB => {}
            0xEC => {}
            0xED => {}
            0xEE => {}
            0xEF => {}
            0xF0 => {}
            0xF1 => {}
            0xF2 => {}
            0xF3 => {}
            0xF4 => {}
            0xF5 => {}
            0xF6 => {}
            0xF7 => {}
            0xF8 => {}
            0xF9 => {}
            0xFA => {}
            0xFB => {}
            0xFC => {}
            0xFD => {}
            0xFE => {}
            0xFF => {}
        }
    }

    pub fn alu_add(&mut self, operand: Registers8b) {
        let (op1, op2) = (self.reg.a, self.reg.read_8b(operand));
        let result = op1.wrapping_add(op2);
        self.reg.a = result;
        self.reg.set_flag(FZ, result == 0);
        self.reg.set_flag(FN, false);
        self.reg.set_flag(FH, (op1 & 0x0F) + (op2 & 0x0F) > 0x0F);
        self.reg.set_flag(FC, (op1 as u16) + (op2 as u16) > 0xFF);
    }
    pub fn alu_adc(&mut self, operand: Registers8b) {
        let carry = self.reg.get_flag(FC) as u8;
        let (op1, op2) = (self.reg.a, self.reg.read_8b(operand));
        let result = op1.wrapping_add(op2).wrapping_add(carry);
        self.reg.a = result;
        self.reg.set_flag(FZ, result == 0);
        self.reg.set_flag(FN, false);
        self.reg
            .set_flag(FH, (op1 & 0x0F) + (op2 & 0x0F) + carry > 0x0F);
        self.reg
            .set_flag(FC, (op1 as u16) + (op2 as u16) + (carry as u16) > 0xFF);
    }
}
