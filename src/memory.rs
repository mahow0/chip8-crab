use crate::font::FONT_TABLE;
use ux::*;
// CHIP-8 was commonly implemented on systems with 4 kB of memory, which we replicate here with an array of bytes

const SIZE: usize = 4096;
const PROGRAM_START: usize = 0x200;

pub struct Memory {
    mem: [u8; SIZE],
}

impl Memory {
    pub fn new() -> Self {
        let mut mem: [u8; SIZE] = [0; SIZE];
        //Copy the contents of the font table into memory
        mem[0..FONT_TABLE.len()].copy_from_slice(&FONT_TABLE);
        Memory { mem: mem }
    }

    pub fn read(&self, addr: u12) -> u8 {
        let wide: u16 = u12::into(addr);
        let index: usize = u16::into(wide);
        self.mem[index]
    }

    pub fn write(&mut self, addr: u12, value: u8) -> () {
        let wide: u16 = u12::into(addr);
        let index: usize = u16::into(wide);
        self.mem[index] = value;
    }

    pub fn load_program(&mut self, data: &[u8]) -> () {
        let index = PROGRAM_START;
        assert!(
            index + data.len() <= (SIZE - PROGRAM_START),
            "program too large"
        );
        self.mem[index..index + data.len()].copy_from_slice(data);
    }
}
