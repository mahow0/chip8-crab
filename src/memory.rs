
use crate::font::FONT_TABLE;
// CHIP-8 was commonly implemented on systems with 4 kB of memory, which we replicate here with an array of bytes

const SIZE : usize = 4096;

pub struct Memory {
    mem : [u8; SIZE]
}

// CHIP-8 memory should be 12-bit addressable, so let's implement a function that checks that an address is safe to use for indexing purposes
pub fn is_twelve_bit(addr : u16) -> bool {
        let shifted_addr: u16 = addr >> 8;
        shifted_addr == 0
}



impl Memory {
    pub fn new() -> Self {
        let mut mem : [u8; SIZE] = [0; SIZE];

        //Copy the contents of the font table into memory
        mem[0..FONT_TABLE.len()].copy_from_slice(&FONT_TABLE); 

        Memory { mem : mem }
    }

    pub fn read(&self, addr : u16) -> u8 {
        if !is_twelve_bit(addr) {
            panic!("Tried to read into memory with an address wider than 12 bits")
        }

        self.mem[usize::from(addr)]
    }

    pub fn write(&mut self, addr : u16, value : u8) -> () {
        if !is_twelve_bit(addr) {
            panic!("Tried to write to memory with an address wider than 12 bits")
        }

        self.mem[usize::from(addr)] = value;
    }
}
