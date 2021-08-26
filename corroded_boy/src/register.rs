#[derive(Copy, Clone)]
pub struct RegisterFile {
    pub a: u8, //Accumulator
    pub f: u8, //Flags
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub pc: u16, //Program Counter
    pub sp: u16, //Stack Pointer
}

pub enum Registers16b {
    AF,
    BC,
    DE,
    HL,
}

pub enum Flag {
    Z = 0b1000000,
    N = 0b0100000,
    H = 0b0010000,
    C = 0b0001000,
    //4 lsb are not used
}

impl RegisterFile {
    pub fn new() -> RegisterFile {
        RegisterFile {
            a: 0,
            f: 0,
            b: 0,
            c: 0,
            e: 0,
            h: 0,
            l: 0,
            sp: 0xFFFE,
            pc: 0x0000,
        }
    }

    pub fn read_16b(&self, regaddr: Registers16b) -> u16 {
        match regaddr {
            Registers16b::AF => (self.a as u16) << 8 | self.f as u16,
            Registers16b::BC => (self.b as u16) << 8 | self.c as u16,
            Registers16b::DE => (self.d as u16) << 8 | self.e as u16,
            Registers16b::HL => (self.h as u16) << 8 | self.l as u16,
        }
    }

    pub fn write_16b(&mut self, regaddr: Registers16b, value: u16) {
        match regaddr {
            Registers16b::AF => {
                self.a = (value >> 8) as u8;
                self.f = (value & 0x00FF) as u8;
            }
            Registers16b::BC => {
                self.b = (value >> 8) as u8;
                self.c = (value & 0x00FF) as u8;
            }
            Registers16b::DE => {
                self.d = (value >> 8) as u8;
                self.e = (value & 0x00FF) as u8;
            }
            Registers16b::HL => {
                self.h = (value >> 8) as u8;
                self.l = (value & 0x00FF) as u8;
            }
        }
    }

    pub fn set_flag(&mut self, flag: Flag, set: bool) {
        if set {
            self.f = self.f | flag;
        } else {
            self.f = self.f & !flag;
        }
    }

    pub fn get_flag(&self, flag: Flag) {
        self.f & flag != 0
    }
}
