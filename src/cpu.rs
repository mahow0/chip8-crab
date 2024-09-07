use crate::memory::Memory;

pub struct CPU {
    ram : Memory, // 4kB of RAM
    stack : Vec<u16>, // stack, comprising of 2-byte values
    pc : u16, // program counter
    index : u16, // index register "I", used to point to addresses in memory
    // TODO: decide whether to include timers here, or lift them to main.rs
    vs : [u8; 16] // general-purpose registers, labeled V0-VF
}


impl CPU {
    pub fn new() -> Self;

    pub fn fetch(self) -> (u8, u8) {
        let byte_1 : u8 = self.ram.read(self.pc);
        let byte_2 : u8 = self.ram.read(self.pc + 1);

        (byte_1, byte_2) 
    } 

    pub fn decode(instr : (u8, u8)) -> ();

    pub fn execute(instr : (u8, u8)) -> ();
}

