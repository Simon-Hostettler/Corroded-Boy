pub struct Memory {}

impl Memory {
    pub fn new() -> Memory {
        Memory {}
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        0
    }

    pub fn write_byte(&self, addr: u16, value: u8) {}
    pub fn read_word(&self, addr: u16) -> u16 {
        0
    }

    pub fn write_word(&self, addr: u16, value: u16) {}
}
