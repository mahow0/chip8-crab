use crate::memory::Memory;
use ux::*;
pub struct CPU {
    ram : Memory, // 4kB of RAM
    vram : [[bool ; 64]; 32],
    stack : Vec<u16>, // stack, comprising of 2-byte values
    pc : u12, // program counter
    index : u12, // index register "I", used to point to addresses in memory
    // TODO: decide whether to include timers here, or lift them to main.rs
    vs : [u8; 16] // general-purpose registers, labeled V0-VF
}


struct NibblePair(u4, u4); 

impl From<u8> for NibblePair {
    fn from(value : u8) -> NibblePair {
        
        let first_nibble : u4 = (value >> 4).try_into().unwrap();
        let second_nibble : u4 = (0b0000_1111 & value).try_into().unwrap();

        NibblePair(first_nibble, second_nibble)
        
    }
}

pub fn nibtrio_2_u12(trio : (u4, u4, u4)) -> u12 {

    let mut twelve : u12 = 0.into();
    let (nib_1, nib_2, nib_3) = trio;

    twelve = twelve  + nib_1.into();
    twelve = twelve << 8;

    twelve = twelve + nib_2.into();
    twelve = twelve << 4;

    twelve = twelve + nib_3.into();

    twelve
}


pub enum Opcode {
    ClearScreen, 
    Jump(u12),
    SetReg(u4, u8),
    AddReg(u4, u8),
    SetI(u12),
    Display(u4, u4, u4),
}


impl CPU {
    pub fn new() -> Self {
        let ram: Memory = Memory::new(); 
        
        CPU {
            ram : ram,
            vram : [[false; 64]; 32],
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

    pub fn decode(instr : (u8, u8)) -> Opcode {
        match instr {
            (0x00, 0xE0) => Opcode::ClearScreen,
            (byte_1 @ 0x10..=0x1F, byte_2) => {
                let nibs_1 : NibblePair = byte_1.into();
                let NibblePair(_, nib_1) = nibs_1;
                let NibblePair(nib_2, nib_3) = byte_2.into();
                Opcode::Jump(nibtrio_2_u12((nib_1, nib_2, nib_3)))
            }
            (byte_1 @ 0x60..=0x6F, byte_2) => {
                let NibblePair(_, nib_1) = byte_1.into();
                Opcode::SetReg(nib_1, byte_2)
            }
            (byte_1 @ 0x70..=0x7F, byte_2) => {
                let NibblePair(_, nib_1) = byte_1.into();
                Opcode::AddReg(nib_1, byte_2)
            }
            (byte_1 @ 0xA0..=0xAF, byte_2) => {
                let NibblePair(_, nib_1) = byte_1.into();
                let NibblePair(nib_2, nib_3) = byte_2.into();
                Opcode::SetI(nibtrio_2_u12((nib_1, nib_2, nib_3)))  
            }
            (byte_1 @ 0xD0..=0xDF, byte_2) => {
                let NibblePair(_, nib_1) = byte_1.into();
                let NibblePair(nib_2, nib_3) = byte_2.into();
                Opcode::Display(nib_1, nib_2, nib_3)
            }
            _ => todo!("more instructions")
        }
    }

    pub fn execute(opcode : Opcode) -> () {
        unimplemented!("execute() needs to be implemented")
    }
}

