use crate::memory::Memory;
use crate::register::Flags::{FC, FH, FN, FZ};
use crate::register::RegisterFile;
use crate::register::Registers16b::{AF, BC, DE, HL, SP};
use crate::register::Registers8b::{A, B, C, D, E, F, H, L};
use crate::register::{Registers16b, Registers8b};

pub struct CPU {
    pub reg: RegisterFile,
    pub mem: Memory,
    pub is_halted: bool,
    pub ime: bool,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            reg: RegisterFile::new(),
            mem: Memory::new(),
            is_halted: false,
            ime: false,
        }
    }

    fn execute(&mut self) {
        let operation = self.fetch_byte();
        match operation {
            0x00 => {} //nop
            0x01 => {
                let val = self.fetch_word();
                self.reg.write_16b(BC, val);
            }
            0x02 => self.mem.write_byte(self.reg.read_16b(BC), self.reg.a),
            0x03 => self
                .reg
                .write_16b(BC, self.reg.read_16b(BC).wrapping_add(1)),
            0x04 => self.alu_inc(B),
            0x05 => self.alu_dec(B),
            0x06 => self.reg.b = self.fetch_byte(),
            0x07 => {
                self.reg.a = self.alu_rlc(self.reg.a);
                self.reg.set_flag(FZ, false);
            }
            0x08 => {
                let val = self.fetch_word();
                self.mem.write_word(val, self.reg.sp);
            }
            0x09 => self.alu_add_16b(BC),
            0x0A => self.reg.a = self.mem.read_byte(self.reg.read_16b(BC)),
            0x0B => self
                .reg
                .write_16b(BC, self.reg.read_16b(BC).wrapping_sub(1)),
            0x0C => self.alu_inc(C),
            0x0D => self.alu_dec(C),
            0x0E => self.reg.c = self.fetch_byte(),
            0x0F => {
                self.reg.a = self.alu_rrc(self.reg.a);
                self.reg.set_flag(FZ, false);
            }
            0x10 => {} //TODO STOP FUNCTION
            0x11 => {
                let val = self.fetch_word();
                self.reg.write_16b(DE, val);
            }
            0x12 => self.mem.write_byte(self.reg.read_16b(DE), self.reg.a),
            0x13 => self
                .reg
                .write_16b(DE, self.reg.read_16b(DE).wrapping_add(1)),
            0x14 => self.alu_inc(D),
            0x15 => self.alu_dec(D),
            0x16 => self.reg.d = self.fetch_byte(),
            0x17 => {
                self.reg.a = self.alu_rl(self.reg.a);
                self.reg.set_flag(FZ, false);
            }
            0x18 => self.jr(true),
            0x19 => self.alu_add_16b(DE),
            0x1A => self.reg.a = self.mem.read_byte(self.reg.read_16b(DE)),
            0x1B => self
                .reg
                .write_16b(DE, self.reg.read_16b(BC).wrapping_sub(1)),
            0x1C => self.alu_inc(E),
            0x1D => self.alu_dec(E),
            0x1E => self.reg.e = self.fetch_byte(),
            0x1F => {
                self.reg.a = self.alu_rr(self.reg.a);
                self.reg.set_flag(FZ, false);
            }
            0x20 => self.jr(!self.reg.get_flag(FZ)),
            0x21 => {
                let val = self.fetch_word();
                self.reg.write_16b(HL, val);
            }
            0x22 => self.mem.write_byte(self.reg.hl_inc(), self.reg.a),
            0x23 => self
                .reg
                .write_16b(HL, self.reg.read_16b(HL).wrapping_add(1)),
            0x24 => self.alu_inc(H),
            0x25 => self.alu_dec(H),
            0x26 => self.reg.h = self.fetch_byte(),
            0x27 => self.alu_daa(),
            0x28 => self.jr(self.reg.get_flag(FZ)),
            0x29 => self.alu_add_16b(HL),
            0x2A => self.reg.a = self.mem.read_byte(self.reg.hl_inc()),
            0x2B => self
                .reg
                .write_16b(HL, self.reg.read_16b(BC).wrapping_sub(1)),
            0x2C => self.alu_inc(L),
            0x2D => self.alu_dec(L),
            0x2E => self.reg.l = self.fetch_byte(),
            0x2F => {
                self.reg.a ^= 0xFF;
                self.reg.set_flag(FN, true);
                self.reg.set_flag(FH, true);
            }
            0x30 => self.jr(!self.reg.get_flag(FC)),
            0x31 => self.reg.sp = self.fetch_word(),
            0x32 => self.mem.write_byte(self.reg.hl_dec(), self.reg.a),
            0x33 => self.reg.sp = self.reg.sp.wrapping_add(1),
            0x34 => self.mem_inc(self.reg.read_16b(HL)),
            0x35 => self.mem_dec(self.reg.read_16b(HL)),
            0x36 => {
                let val = self.fetch_byte();
                self.mem.write_byte(self.reg.read_16b(HL), val);
            }
            0x37 => self
                .reg
                .set_flags(self.reg.get_flag(FZ), false, false, true),
            0x38 => self.jr(self.reg.get_flag(FC)),
            0x39 => self.alu_add_16b(SP),
            0x3A => self.reg.a = self.mem.read_byte(self.reg.hl_dec()),
            0x3B => self.reg.sp = self.reg.sp.wrapping_sub(1),
            0x3C => self.alu_inc(A),
            0x3D => self.alu_dec(A),
            0x3E => self.reg.a = self.fetch_byte(),
            0x3F => self
                .reg
                .set_flags(self.reg.get_flag(FZ), false, false, !self.reg.get_flag(FC)),
            0x40 => self.reg.b = self.reg.b,
            0x41 => self.reg.b = self.reg.c,
            0x42 => self.reg.b = self.reg.d,
            0x43 => self.reg.b = self.reg.e,
            0x44 => self.reg.b = self.reg.h,
            0x45 => self.reg.b = self.reg.l,
            0x46 => self.reg.b = self.mem.read_byte(self.reg.read_16b(HL)),
            0x47 => self.reg.b = self.reg.a,
            0x48 => self.reg.c = self.reg.b,
            0x49 => self.reg.c = self.reg.c,
            0x4B => self.reg.c = self.reg.d,
            0x4C => self.reg.c = self.reg.e,
            0x4D => self.reg.c = self.reg.h,
            0x4A => self.reg.c = self.reg.l,
            0x4E => self.reg.c = self.mem.read_byte(self.reg.read_16b(HL)),
            0x4F => self.reg.c = self.reg.a,
            0x50 => self.reg.d = self.reg.b,
            0x51 => self.reg.d = self.reg.c,
            0x52 => self.reg.d = self.reg.d,
            0x53 => self.reg.d = self.reg.e,
            0x54 => self.reg.d = self.reg.h,
            0x55 => self.reg.d = self.reg.l,
            0x56 => self.reg.d = self.mem.read_byte(self.reg.read_16b(HL)),
            0x57 => self.reg.d = self.reg.a,
            0x58 => self.reg.e = self.reg.b,
            0x59 => self.reg.e = self.reg.c,
            0x5A => self.reg.e = self.reg.d,
            0x5B => self.reg.e = self.reg.e,
            0x5C => self.reg.e = self.reg.h,
            0x5D => self.reg.e = self.reg.l,
            0x5E => self.reg.e = self.mem.read_byte(self.reg.read_16b(HL)),
            0x5F => self.reg.e = self.reg.a,
            0x60 => self.reg.h = self.reg.b,
            0x61 => self.reg.h = self.reg.c,
            0x62 => self.reg.h = self.reg.d,
            0x63 => self.reg.h = self.reg.e,
            0x64 => self.reg.h = self.reg.h,
            0x65 => self.reg.h = self.reg.l,
            0x66 => self.reg.h = self.mem.read_byte(self.reg.read_16b(HL)),
            0x67 => self.reg.h = self.reg.a,
            0x68 => self.reg.l = self.reg.b,
            0x69 => self.reg.l = self.reg.c,
            0x6A => self.reg.l = self.reg.d,
            0x6B => self.reg.l = self.reg.e,
            0x6C => self.reg.l = self.reg.h,
            0x6D => self.reg.l = self.reg.l,
            0x6E => self.reg.l = self.mem.read_byte(self.reg.read_16b(HL)),
            0x6F => self.reg.l = self.reg.a,
            0x70 => self.mem.write_byte(self.reg.read_16b(HL), self.reg.b),
            0x71 => self.mem.write_byte(self.reg.read_16b(HL), self.reg.c),
            0x72 => self.mem.write_byte(self.reg.read_16b(HL), self.reg.d),
            0x73 => self.mem.write_byte(self.reg.read_16b(HL), self.reg.e),
            0x74 => self.mem.write_byte(self.reg.read_16b(HL), self.reg.h),
            0x75 => self.mem.write_byte(self.reg.read_16b(HL), self.reg.l),
            0x76 => self.is_halted = true,
            0x77 => self.mem.write_byte(self.reg.read_16b(HL), self.reg.a),
            0x78 => self.reg.a = self.reg.b,
            0x79 => self.reg.a = self.reg.c,
            0x7A => self.reg.a = self.reg.d,
            0x7B => self.reg.a = self.reg.e,
            0x7C => self.reg.a = self.reg.h,
            0x7D => self.reg.a = self.reg.l,
            0x7E => self.reg.a = self.mem.read_byte(self.reg.read_16b(HL)),
            0x7F => self.reg.a = self.reg.a,
            0x80 => self.alu_add(self.reg.b),
            0x81 => self.alu_add(self.reg.c),
            0x82 => self.alu_add(self.reg.d),
            0x83 => self.alu_add(self.reg.e),
            0x84 => self.alu_add(self.reg.h),
            0x85 => self.alu_add(self.reg.l),
            0x86 => self.alu_add(self.mem.read_byte(self.reg.read_16b(HL))),
            0x87 => self.alu_add(self.reg.a),
            0x88 => self.alu_adc(self.reg.b),
            0x89 => self.alu_adc(self.reg.c),
            0x8A => self.alu_adc(self.reg.d),
            0x8B => self.alu_adc(self.reg.e),
            0x8C => self.alu_adc(self.reg.h),
            0x8D => self.alu_adc(self.reg.l),
            0x8E => self.alu_add(self.mem.read_byte(self.reg.read_16b(HL))),
            0x8F => self.alu_adc(self.reg.a),
            0x90 => self.alu_sub(self.reg.b),
            0x91 => self.alu_sub(self.reg.c),
            0x92 => self.alu_sub(self.reg.d),
            0x93 => self.alu_sub(self.reg.e),
            0x94 => self.alu_sub(self.reg.h),
            0x95 => self.alu_sub(self.reg.l),
            0x96 => self.alu_sub(self.mem.read_byte(self.reg.read_16b(HL))),
            0x97 => self.alu_sub(self.reg.a),
            0x98 => self.alu_sbc(self.reg.b),
            0x99 => self.alu_sbc(self.reg.c),
            0x9A => self.alu_sbc(self.reg.d),
            0x9B => self.alu_sbc(self.reg.e),
            0x9C => self.alu_sbc(self.reg.h),
            0x9D => self.alu_sbc(self.reg.l),
            0x9E => self.alu_sbc(self.mem.read_byte(self.reg.read_16b(HL))),
            0x9F => self.alu_sbc(self.reg.a),
            0xA0 => self.alu_and(self.reg.b),
            0xA1 => self.alu_and(self.reg.c),
            0xA2 => self.alu_and(self.reg.d),
            0xA3 => self.alu_and(self.reg.e),
            0xA4 => self.alu_and(self.reg.h),
            0xA5 => self.alu_and(self.reg.l),
            0xA6 => self.alu_and(self.mem.read_byte(self.reg.read_16b(HL))),
            0xA7 => self.alu_and(self.reg.a),
            0xA8 => self.alu_xor(self.reg.b),
            0xA9 => self.alu_xor(self.reg.c),
            0xAA => self.alu_xor(self.reg.d),
            0xAB => self.alu_xor(self.reg.e),
            0xAC => self.alu_xor(self.reg.h),
            0xAD => self.alu_xor(self.reg.l),
            0xAE => self.alu_xor(self.mem.read_byte(self.reg.read_16b(HL))),
            0xAF => self.alu_xor(self.reg.a),
            0xB0 => self.alu_or(self.reg.b),
            0xB1 => self.alu_or(self.reg.c),
            0xB2 => self.alu_or(self.reg.d),
            0xB3 => self.alu_or(self.reg.e),
            0xB4 => self.alu_or(self.reg.h),
            0xB5 => self.alu_or(self.reg.l),
            0xB6 => self.alu_or(self.mem.read_byte(self.reg.read_16b(HL))),
            0xB7 => self.alu_or(self.reg.a),
            0xB8 => self.alu_cp(self.reg.b),
            0xB9 => self.alu_cp(self.reg.c),
            0xBA => self.alu_cp(self.reg.d),
            0xBB => self.alu_cp(self.reg.e),
            0xBC => self.alu_cp(self.reg.h),
            0xBD => self.alu_cp(self.reg.l),
            0xBE => self.alu_cp(self.mem.read_byte(self.reg.read_16b(HL))),
            0xBF => self.alu_cp(self.reg.a),
            0xC0 => self.ret(!self.reg.get_flag(FZ)),
            0xC1 => {
                let word = self.pop_stack();
                self.reg.write_16b(BC, word);
            }
            0xC2 => self.jp(!self.reg.get_flag(FZ)),
            0xC3 => self.jp(true),
            0xC4 => self.call(!self.reg.get_flag(FZ)),
            0xC5 => {
                let word = self.reg.read_16b(BC);
                self.push_stack(word);
            }
            0xC6 => {
                let val = self.fetch_byte();
                self.alu_add(val);
            }
            0xC7 => self.rst(0x00),
            0xC8 => self.ret(self.reg.get_flag(FZ)),
            0xC9 => self.ret(true),
            0xCA => self.jp(self.reg.get_flag(FZ)),
            0xCB => self.execute_cb(),
            0xCC => self.call(self.reg.get_flag(FZ)),
            0xCD => self.call(true),
            0xCE => {
                let val = self.fetch_byte();
                self.alu_adc(val);
            }
            0xCF => self.rst(0x08),
            0xD0 => self.ret(!self.reg.get_flag(FC)),
            0xD1 => {
                let word = self.pop_stack();
                self.reg.write_16b(DE, word);
            }
            0xD2 => self.jp(!self.reg.get_flag(FC)),
            0xD3 => {} //unused
            0xD4 => self.call(!self.reg.get_flag(FC)),
            0xD5 => {
                let word = self.reg.read_16b(DE);
                self.push_stack(word);
            }
            0xD6 => {
                let val = self.fetch_byte();
                self.alu_sub(val);
            }
            0xD7 => self.rst(0x10),
            0xD8 => self.ret(self.reg.get_flag(FC)),
            0xD9 => {
                self.ret(true);
                self.ime = true;
            }
            0xDA => self.jp(self.reg.get_flag(FC)),
            0xDB => {} //unused
            0xDC => self.call(self.reg.get_flag(FC)),
            0xDD => {} //unused
            0xDE => {
                let val = self.fetch_byte();
                self.alu_sbc(val);
            }
            0xDF => self.rst(0x18),
            0xE0 => {
                let addr = self.fetch_byte() as u16 + 0xFF00;
                self.mem.write_byte(addr, self.reg.a);
            }
            0xE1 => {
                let word = self.pop_stack();
                self.reg.write_16b(HL, word);
            }
            0xE2 => {
                let addr = self.reg.c as u16 + 0xFF00;
                self.mem.write_byte(addr, self.reg.a);
            }
            0xE3 => {} //unused
            0xE4 => {} //unused
            0xE5 => {
                let word = self.reg.read_16b(HL);
                self.push_stack(word);
            }
            0xE6 => {
                let val = self.fetch_byte();
                self.alu_and(val);
            }
            0xE7 => self.rst(0x20),
            0xE8 => self.reg.sp = self.alu_add_imm(self.reg.pc),
            0xE9 => self.reg.pc = self.reg.read_16b(HL),
            0xEA => {
                let addr = self.fetch_word();
                self.mem.write_byte(addr, self.reg.a);
            }
            0xEB => {} //unused
            0xEC => {} //unused
            0xED => {} //unused
            0xEE => {
                let val = self.fetch_byte();
                self.alu_xor(val);
            }
            0xEF => self.rst(0x28),
            0xF0 => {
                let addr = self.reg.a as u16 + 0xFF00;
                self.reg.a = self.mem.read_byte(addr);
            }
            0xF1 => {
                let word = self.pop_stack();
                self.reg.write_16b(AF, word);
            }
            0xF2 => {
                let addr = self.reg.c as u16 + 0xFF00;
                self.reg.a = self.mem.read_byte(addr);
            }
            0xF3 => self.ime = false,
            0xF4 => {} //unused
            0xF5 => {
                let word = self.reg.read_16b(AF);
                self.push_stack(word);
            }
            0xF6 => {
                let val = self.fetch_byte();
                self.alu_or(val);
            }
            0xF7 => self.rst(0x30),
            0xF8 => {
                let val = self.alu_add_imm(self.reg.pc);
                self.reg.write_16b(HL, val);
            }
            0xF9 => self.reg.sp = self.reg.read_16b(HL),
            0xFA => {
                let addr = self.fetch_word();
                self.reg.a = self.mem.read_byte(addr);
            }
            0xFB => self.ime = true,
            0xFC => {} //unused
            0xFD => {} //unused
            0xFE => {
                let val = self.fetch_byte();
                self.alu_cp(val);
            }
            0xFF => self.rst(0x38),
        }
    }

    fn execute_cb(&mut self) {
        let operation = self.fetch_byte();
        match operation {
            0x00 => self.reg.b = self.alu_rlc(self.reg.b),
            0x01 => self.reg.c = self.alu_rlc(self.reg.c),
            0x02 => self.reg.d = self.alu_rlc(self.reg.d),
            0x03 => self.reg.e = self.alu_rlc(self.reg.e),
            0x04 => self.reg.h = self.alu_rlc(self.reg.h),
            0x05 => self.reg.l = self.alu_rlc(self.reg.l),
            0x06 => {
                let result = self.alu_rlc(self.mem.read_byte(self.reg.read_16b(HL)));
                self.mem.write_byte(self.reg.read_16b(HL), result);
            }
            0x07 => self.reg.a = self.alu_rlc(self.reg.a),
            0x08 => self.reg.b = self.alu_rrc(self.reg.b),
            0x09 => self.reg.c = self.alu_rrc(self.reg.c),
            0x0A => self.reg.d = self.alu_rrc(self.reg.d),
            0x0B => self.reg.e = self.alu_rrc(self.reg.e),
            0x0C => self.reg.h = self.alu_rrc(self.reg.h),
            0x0D => self.reg.l = self.alu_rrc(self.reg.l),
            0x0E => {
                let result = self.alu_rrc(self.mem.read_byte(self.reg.read_16b(HL)));
                self.mem.write_byte(self.reg.read_16b(HL), result);
            }
            0x0F => self.reg.a = self.alu_rrc(self.reg.a),
            0x10 => self.reg.b = self.alu_rl(self.reg.b),
            0x11 => self.reg.c = self.alu_rl(self.reg.c),
            0x12 => self.reg.d = self.alu_rl(self.reg.d),
            0x13 => self.reg.e = self.alu_rl(self.reg.e),
            0x14 => self.reg.h = self.alu_rl(self.reg.h),
            0x15 => self.reg.l = self.alu_rl(self.reg.l),
            0x16 => {
                let result = self.alu_rl(self.mem.read_byte(self.reg.read_16b(HL)));
                self.mem.write_byte(self.reg.read_16b(HL), result);
            }
            0x17 => self.reg.a = self.alu_rl(self.reg.a),
            0x18 => self.reg.b = self.alu_rr(self.reg.b),
            0x19 => self.reg.c = self.alu_rr(self.reg.c),
            0x1A => self.reg.d = self.alu_rr(self.reg.d),
            0x1B => self.reg.e = self.alu_rr(self.reg.e),
            0x1C => self.reg.h = self.alu_rr(self.reg.h),
            0x1D => self.reg.l = self.alu_rr(self.reg.l),
            0x1E => {
                let result = self.alu_rr(self.mem.read_byte(self.reg.read_16b(HL)));
                self.mem.write_byte(self.reg.read_16b(HL), result);
            }
            0x1F => self.reg.a = self.alu_rr(self.reg.a),
            0x20 => self.reg.b = self.alu_sla(self.reg.b),
            0x21 => self.reg.c = self.alu_sla(self.reg.c),
            0x22 => self.reg.d = self.alu_sla(self.reg.d),
            0x23 => self.reg.e = self.alu_sla(self.reg.e),
            0x24 => self.reg.h = self.alu_sla(self.reg.h),
            0x25 => self.reg.l = self.alu_sla(self.reg.l),
            0x26 => {
                let result = self.alu_sla(self.mem.read_byte(self.reg.read_16b(HL)));
                self.mem.write_byte(self.reg.read_16b(HL), result);
            }
            0x27 => self.reg.a = self.alu_sla(self.reg.a),
            0x28 => self.reg.b = self.alu_sra(self.reg.b),
            0x29 => self.reg.c = self.alu_sra(self.reg.c),
            0x2A => self.reg.d = self.alu_sra(self.reg.d),
            0x2B => self.reg.e = self.alu_sra(self.reg.e),
            0x2C => self.reg.h = self.alu_sra(self.reg.h),
            0x2D => self.reg.l = self.alu_sra(self.reg.l),
            0x2E => {
                let result = self.alu_sra(self.mem.read_byte(self.reg.read_16b(HL)));
                self.mem.write_byte(self.reg.read_16b(HL), result);
            }
            0x2F => self.reg.a = self.alu_sra(self.reg.a),
            0x30 => self.reg.b = self.alu_swap(self.reg.b),
            0x31 => self.reg.c = self.alu_swap(self.reg.c),
            0x32 => self.reg.d = self.alu_swap(self.reg.d),
            0x33 => self.reg.e = self.alu_swap(self.reg.e),
            0x34 => self.reg.h = self.alu_swap(self.reg.h),
            0x35 => self.reg.l = self.alu_swap(self.reg.l),
            0x36 => {
                let result = self.alu_swap(self.mem.read_byte(self.reg.read_16b(HL)));
                self.mem.write_byte(self.reg.read_16b(HL), result);
            }
            0x37 => self.reg.a = self.alu_swap(self.reg.a),
            0x38 => self.reg.b = self.alu_srl(self.reg.b),
            0x39 => self.reg.c = self.alu_srl(self.reg.c),
            0x3A => self.reg.d = self.alu_srl(self.reg.d),
            0x3B => self.reg.e = self.alu_srl(self.reg.e),
            0x3C => self.reg.h = self.alu_srl(self.reg.h),
            0x3D => self.reg.l = self.alu_srl(self.reg.l),
            0x3E => {
                let result = self.alu_srl(self.mem.read_byte(self.reg.read_16b(HL)));
                self.mem.write_byte(self.reg.read_16b(HL), result);
            }
            0x3F => self.reg.a = self.alu_srl(self.reg.a),
            0x40 => self.test_bit(self.reg.b, 0),
            0x41 => self.test_bit(self.reg.c, 0),
            0x42 => self.test_bit(self.reg.d, 0),
            0x43 => self.test_bit(self.reg.e, 0),
            0x44 => self.test_bit(self.reg.h, 0),
            0x45 => self.test_bit(self.reg.l, 0),
            0x46 => self.test_bit(self.mem.read_byte(self.reg.read_16b(HL)), 0),
            0x47 => self.test_bit(self.reg.a, 0),
            0x48 => self.test_bit(self.reg.b, 1),
            0x49 => self.test_bit(self.reg.c, 1),
            0x4A => self.test_bit(self.reg.d, 1),
            0x4B => self.test_bit(self.reg.e, 1),
            0x4C => self.test_bit(self.reg.h, 1),
            0x4D => self.test_bit(self.reg.l, 1),
            0x4E => self.test_bit(self.mem.read_byte(self.reg.read_16b(HL)), 1),
            0x4F => self.test_bit(self.reg.a, 1),
            0x50 => self.test_bit(self.reg.b, 2),
            0x51 => self.test_bit(self.reg.c, 2),
            0x52 => self.test_bit(self.reg.d, 2),
            0x53 => self.test_bit(self.reg.e, 2),
            0x54 => self.test_bit(self.reg.h, 2),
            0x55 => self.test_bit(self.reg.l, 2),
            0x56 => self.test_bit(self.mem.read_byte(self.reg.read_16b(HL)), 2),
            0x57 => self.test_bit(self.reg.a, 2),
            0x58 => self.test_bit(self.reg.b, 3),
            0x59 => self.test_bit(self.reg.c, 3),
            0x5A => self.test_bit(self.reg.d, 3),
            0x5B => self.test_bit(self.reg.e, 3),
            0x5C => self.test_bit(self.reg.h, 3),
            0x5D => self.test_bit(self.reg.l, 3),
            0x5E => self.test_bit(self.mem.read_byte(self.reg.read_16b(HL)), 3),
            0x5F => self.test_bit(self.reg.a, 3),
            0x60 => self.test_bit(self.reg.b, 4),
            0x61 => self.test_bit(self.reg.c, 4),
            0x62 => self.test_bit(self.reg.d, 4),
            0x63 => self.test_bit(self.reg.e, 4),
            0x64 => self.test_bit(self.reg.h, 4),
            0x65 => self.test_bit(self.reg.l, 4),
            0x66 => self.test_bit(self.mem.read_byte(self.reg.read_16b(HL)), 4),
            0x67 => self.test_bit(self.reg.a, 4),
            0x68 => self.test_bit(self.reg.b, 5),
            0x69 => self.test_bit(self.reg.c, 5),
            0x6A => self.test_bit(self.reg.d, 5),
            0x6B => self.test_bit(self.reg.e, 5),
            0x6C => self.test_bit(self.reg.h, 5),
            0x6D => self.test_bit(self.reg.l, 5),
            0x6E => self.test_bit(self.mem.read_byte(self.reg.read_16b(HL)), 5),
            0x6F => self.test_bit(self.reg.a, 5),
            0x70 => self.test_bit(self.reg.b, 6),
            0x71 => self.test_bit(self.reg.c, 6),
            0x72 => self.test_bit(self.reg.d, 6),
            0x73 => self.test_bit(self.reg.e, 6),
            0x74 => self.test_bit(self.reg.h, 6),
            0x75 => self.test_bit(self.reg.l, 6),
            0x76 => self.test_bit(self.mem.read_byte(self.reg.read_16b(HL)), 6),
            0x77 => self.test_bit(self.reg.a, 6),
            0x78 => self.test_bit(self.reg.b, 7),
            0x79 => self.test_bit(self.reg.c, 7),
            0x7A => self.test_bit(self.reg.d, 7),
            0x7B => self.test_bit(self.reg.e, 7),
            0x7C => self.test_bit(self.reg.h, 7),
            0x7D => self.test_bit(self.reg.l, 7),
            0x7E => self.test_bit(self.mem.read_byte(self.reg.read_16b(HL)), 7),
            0x7F => self.test_bit(self.reg.a, 7),
            0x80 => {}
            0x81 => {}
            0x82 => {}
            0x83 => {}
            0x84 => {}
            0x85 => {}
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

    fn fetch_byte(&mut self) -> u8 {
        let byte = self.mem.read_byte(self.reg.sp);
        self.reg.sp = self.reg.sp.wrapping_add(1);
        byte
    }

    fn fetch_word(&mut self) -> u16 {
        let word = self.mem.read_word(self.reg.sp);
        self.reg.sp = self.reg.sp.wrapping_add(2);
        word
    }

    fn push_stack(&mut self, value: u16) {
        self.reg.sp -= 2;
        self.mem.write_word(self.reg.sp, value);
    }
    fn pop_stack(&mut self) -> u16 {
        let word = self.mem.read_word(self.reg.sp);
        self.reg.sp += 2;
        word
    }

    fn alu_add(&mut self, operand: u8) {
        let op1 = self.reg.a;
        let result = op1.wrapping_add(operand);
        self.reg.a = result;
        self.reg.set_flag(FZ, result == 0);
        self.reg.set_flag(FN, false);
        self.reg
            .set_flag(FH, (op1 & 0x0F) + (operand & 0x0F) > 0x0F);
        self.reg
            .set_flag(FC, (op1 as u16) + (operand as u16) > 0xFF);
    }

    fn alu_add_16b(&mut self, operand: Registers16b) {
        let (val1, val2) = (self.reg.read_16b(HL), self.reg.read_16b(operand));
        let result = val1.wrapping_add(val2);
        self.reg.set_flag(FN, false);
        self.reg
            .set_flag(FH, val1 & 0x07FF > 0x07FF - (val2 & 0x07FF));
        self.reg.set_flag(FC, (val1 as u32 + val2 as u32) > 0xFFFF);
        self.reg.write_16b(HL, result);
    }

    fn alu_add_imm(&mut self, operand: u16) -> u16 {
        let imm = self.fetch_byte() as u16;
        let res = operand.wrapping_add(imm);
        self.reg.set_flag(FH, (operand & 0xF) + (imm & 0xF) > 0xF);
        self.reg
            .set_flag(FH, (operand & 0xFF) + (imm & 0xFF) > 0xFF);
        res
    }

    fn alu_adc(&mut self, operand: u8) {
        let carry = self.reg.get_flag(FC) as u8;
        let op1 = self.reg.a;
        let result = op1.wrapping_add(operand).wrapping_add(carry);
        self.reg.a = result;
        self.reg.set_flag(FZ, result == 0);
        self.reg.set_flag(FN, false);
        self.reg
            .set_flag(FH, (op1 & 0x0F) + (operand & 0x0F) + carry > 0x0F);
        self.reg
            .set_flag(FC, (op1 as u16) + (operand as u16) + (carry as u16) > 0xFF);
    }

    fn alu_sub(&mut self, operand: u8) {
        let op1 = self.reg.a;
        let result = op1.wrapping_sub(operand);
        self.reg.a = result;
        self.reg.set_flag(FZ, result == 0);
        self.reg.set_flag(FN, true);
        self.reg.set_flag(FH, (op1 & 0x0F) < (operand & 0x0F));
        self.reg.set_flag(FC, (op1 as u16) < (operand as u16));
    }

    fn alu_sbc(&mut self, operand: u8) {
        let carry = self.reg.get_flag(FC) as u8;
        let op1 = self.reg.a;
        let result = op1.wrapping_sub(operand).wrapping_sub(carry);
        self.reg.a = result;
        self.reg.set_flag(FZ, result == 0);
        self.reg.set_flag(FN, true);
        self.reg
            .set_flag(FH, (op1 & 0x0F) < (operand & 0x0F) + carry);
        self.reg
            .set_flag(FC, (op1 as u16) < (operand as u16) + (carry as u16));
    }

    fn alu_and(&mut self, operand: u8) {
        let result = self.reg.a & operand;
        self.reg.a = result;
        self.reg.set_flags(result == 0, false, true, false);
    }

    fn alu_xor(&mut self, operand: u8) {
        let result = self.reg.a ^ operand;
        self.reg.a = result;
        self.reg.set_flags(result == 0, false, false, false);
    }

    fn alu_or(&mut self, operand: u8) {
        let result = self.reg.a | operand;
        self.reg.a = result;
        self.reg.set_flags(result == 0, false, false, false);
    }

    fn alu_cp(&mut self, operand: u8) {
        let op1 = self.reg.a;
        let result = op1.wrapping_sub(operand);
        self.reg.set_flag(FZ, result == 0);
        self.reg.set_flag(FN, true);
        self.reg.set_flag(FH, (op1 & 0x0F) < (operand & 0x0F));
        self.reg.set_flag(FC, (op1 as u16) < (operand as u16));
    }

    fn alu_inc(&mut self, reg: Registers8b) {
        let result = self.reg.read_8b(&reg).wrapping_add(1);
        self.reg.set_flag(FZ, result == 0);
        self.reg.set_flag(FN, false);
        self.reg
            .set_flag(FH, (self.reg.read_8b(&reg) as u16) + 1 > 0x0F);
        self.reg.write_8b(&reg, result);
    }

    fn alu_dec(&mut self, reg: Registers8b) {
        let result = self.reg.read_8b(&reg).wrapping_sub(1);
        self.reg.set_flag(FZ, result == 0);
        self.reg.set_flag(FN, true);
        self.reg.set_flag(FH, self.reg.read_8b(&reg) == 0);
        self.reg.write_8b(&reg, result);
    }

    fn alu_daa(&mut self) {
        let mut val = self.reg.a;
        let mut correction = 0;
        let (flag_n, flag_h, flag_c) = (
            self.reg.get_flag(FN),
            self.reg.get_flag(FH),
            self.reg.get_flag(FC),
        );
        if flag_h || (!flag_n && (val & 0xf) > 9) {
            correction |= 0x6;
        }
        if flag_c || (!flag_n && val > 0x99) {
            correction |= 0x60;
        }
        if flag_n {
            val = val.wrapping_sub(correction);
        } else {
            val = val.wrapping_add(correction);
        }
        self.reg
            .set_flags(val == 0, flag_n, false, correction >= 0x60);
        self.reg.a = val;
    }

    fn mem_inc(&mut self, addr: u16) {
        let operand = self.mem.read_byte(addr);
        let result = operand.wrapping_add(1);
        self.reg.set_flag(FZ, result == 0);
        self.reg.set_flag(FN, false);
        self.reg.set_flag(FH, (operand as u16) + 1 > 0x0F);
        self.mem.write_byte(addr, result);
    }

    fn mem_dec(&mut self, addr: u16) {
        let operand = self.mem.read_byte(addr);
        let result = operand.wrapping_sub(1);
        self.reg.set_flag(FZ, result == 0);
        self.reg.set_flag(FN, true);
        self.reg.set_flag(FH, operand == 0);
        self.mem.write_byte(addr, result);
    }

    fn alu_rlc(&mut self, operand: u8) -> u8 {
        let msb = (operand & 0x80) >> 7;
        let result = (operand << 1) | msb;
        self.reg.set_flags(result == 0, false, false, msb != 0);
        result
    }

    fn alu_rl(&mut self, operand: u8) -> u8 {
        let msb = (operand & 0x80) >> 7;
        let result = (operand << 1) | self.reg.get_flag(FC) as u8;
        self.reg.set_flags(result == 0, false, false, msb != 0);
        result
    }

    fn alu_rrc(&mut self, operand: u8) -> u8 {
        let lsb = (operand & 0x01) << 7;
        let result = (operand >> 1) | lsb;
        self.reg.set_flags(result == 0, false, false, lsb != 0);
        result
    }

    fn alu_rr(&mut self, operand: u8) -> u8 {
        let lsb = (operand & 0x01) << 7;
        let result = (operand >> 1) | ((self.reg.get_flag(FC) as u8) << 7);
        self.reg.set_flags(result == 0, false, false, lsb != 0);
        result
    }

    fn alu_sla(&mut self, operand: u8) -> u8 {
        let msb = (operand & 0x80) >> 7;
        let result = operand << 1;
        self.reg.set_flags(result == 0, false, false, msb != 0);
        result
    }

    fn alu_sra(&mut self, operand: u8) -> u8 {
        let lsb = operand & 0x01;
        let result = (operand >> 1) | (operand & 0x80);
        self.reg.set_flags(result == 0, false, false, lsb != 0);
        result
    }

    fn alu_srl(&mut self, operand: u8) -> u8 {
        let lsb = operand & 0x01;
        let result = operand >> 1;
        self.reg.set_flags(result == 0, false, false, lsb != 0);
        result
    }

    fn alu_swap(&mut self, operand: u8) -> u8 {
        self.reg.set_flags(operand == 0, false, false, false);
        (operand << 4) | (operand >> 4)
    }

    fn test_bit(&mut self, operand: u8, bit: u8) {
        let result = operand & (1 << bit);
        self.reg
            .set_flags(result == 0, false, true, self.reg.get_flag(FC));
    }
    fn reset_bit(&self, operand: u8, bit: u8) -> u8 {
        operand & !(1 << bit)
    }

    fn set_bit(&self, operand: u8, bit: u8) -> u8 {
        operand | (1 << bit)
    }

    fn jr(&mut self, condition: bool) {
        if condition {
            self.reg.pc = self.reg.pc.wrapping_add(self.fetch_byte() as u16);
        } else {
            self.reg.pc = self.reg.pc.wrapping_add(1);
        }
    }

    fn ret(&mut self, condition: bool) {
        if condition {
            self.reg.pc = self.pop_stack();
        }
    }

    fn jp(&mut self, condition: bool) {
        if condition {
            self.reg.pc = self.fetch_word();
        } else {
            self.reg.pc = self.reg.pc.wrapping_add(2);
        }
    }

    fn call(&mut self, condition: bool) {
        if condition {
            self.push_stack(self.reg.pc);
            self.reg.pc = self.fetch_word();
        } else {
            self.reg.pc = self.reg.pc.wrapping_add(2);
        }
    }

    fn rst(&mut self, pointer: u8) {
        self.push_stack(self.reg.pc);
        self.reg.pc = pointer as u16;
    }
}
