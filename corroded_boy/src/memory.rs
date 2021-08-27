pub struct Memory {}

impl Memory {
    pub fn new() -> Memory {
        Memory {}
    }

    pub fn rb(&self, addr: u16) -> u8 {
        0
    }

    pub fn wb(&self, addr: u16, value: u8) {}
}
