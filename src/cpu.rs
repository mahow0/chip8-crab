use crate::memory::Memory;
use ux::*;
pub struct CPU {
    ram : Memory, // 4kB of RAM
    stack : Vec<u16>, // stack, comprising of 2-byte values
    pc : u12, // program counter
    index : u12, // index register "I", used to point to addresses in memory
    // TODO: decide whether to include timers here, or lift them to main.rs
    vs : [u8; 16] // general-purpose registers, labeled V0-VF
}

pub enum Opcode {
    ClearScreen, 
    Jump(u12),
    SetReg(u8),
    AddReg(u4, u8),
    SetI(u12),
    Draw(u4, u4, u4),
}
impl CPU {
    pub fn new() -> Self {
        let ram: Memory = Memory::new(); 
        
        CPU {
            ram : ram,
            stack : vec![0; 16],
            pc : 0x200.into(),
            index : 0x0.into(),
            vs : [0; 16]
        }
    }

    
    pub fn fetch(&mut self) -> (u8, u8) {
        let byte_1 : u8 = self.ram.read(self.pc);
        let byte_2 : u8 = self.ram.read(self.pc + 1.into());
        
        self.pc = self.pc + 2.into();
        (byte_1, byte_2)
    } 

    pub fn decode(instr : (u4, u4, u4, u4)) -> Opcode;

    pub fn execute(opcode : Opcode) -> ();
}

