use crate::memory::Memory;
use crate::error::*;
use ux::*;

const HEIGHT: usize = 32;
const WIDTH: usize = 64;

const HEIGHT_U8: u8 = 32;
const WIDTH_U8: u8 = 64;

pub struct CPU {
    ram: Memory,                   // 4kB of RAM
    vram: [[bool; HEIGHT]; WIDTH], //vram containing pixel values, stored in column-major order
    stack: Vec<u16>,               // stack, comprising of 2-byte values
    pc: u12,                       // program counter
    index: u12,                    // index register "I", used to point to addresses in memory
    // TODO: decide whether to include timers here, or lift them to main.rs
    pub vs: [u8; 16], // general-purpose registers, labeled V0-VF
}

struct NibblePair(u4, u4);

impl From<u8> for NibblePair {
    fn from(value: u8) -> NibblePair {
        let first_nibble: u4 = (value >> 4).try_into().unwrap();
        let second_nibble: u4 = (0b0000_1111 & value).try_into().unwrap();

        NibblePair(first_nibble, second_nibble)
    }
}

pub fn nibtrio_2_u12(trio: (u4, u4, u4)) -> u12 {
    let mut twelve: u12 = 0.into();
    let (nib_1, nib_2, nib_3) = trio;

    twelve = twelve + nib_1.into();
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
            ram: ram,
            vram: [[false; HEIGHT]; WIDTH],
            stack: vec![0; 16],
            pc: (0x200u16).try_into().unwrap(),
            index: 0x0.into(),
            vs: [0; 16],
        }
    }

    pub fn fetch(&mut self) -> (u8, u8) {
        let byte_1: u8 = self.ram.read(self.pc);
        let byte_2: u8 = self.ram.read(self.pc + 1.into());

        self.pc = self.pc + 2.into();

        (byte_1, byte_2)
    }

    pub fn decode(&self, instr: (u8, u8)) -> Opcode {
        self.try_decode(instr).expect("Could not parse {instr:?} to opcode")
    }

    pub fn try_decode(&self, instr: (u8, u8)) -> Result<Opcode> {
        let opcode = match instr {
            (0x00, 0xE0) => Opcode::ClearScreen,
            (byte_1 @ 0x10..=0x1F, byte_2) => {
                let nibs_1: NibblePair = byte_1.into();
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
            _ => return Err(Chip8Error::DecodeError {
                instr,
                reason: "No decoding implementation found for this hex range".to_string(),
            }),
        };
        Ok(opcode)
    }

    pub fn execute(&mut self, opcode: Opcode) -> () {
        match opcode {
            Opcode::ClearScreen => self.op_00e0(),
            Opcode::Jump(addr) => self.op_1nnn(addr),
            Opcode::SetReg(reg, value) => self.op_6xnn(reg, value),
            Opcode::AddReg(reg, value) => self.op_7xnn(reg, value),
            Opcode::SetI(addr) => self.op_annn(addr),
            Opcode::Display(x, y, n) => self.op_dxyn(x, y, n),
        }
    }

    fn op_00e0(&mut self) {
        for i in 0..WIDTH {
            self.vram[i] = [false; HEIGHT];
        }
    }
    fn op_1nnn(&mut self, nnn: u12) {
        self.pc = nnn;
    }

    fn op_6xnn(&mut self, x: u4, nn: u8) {
        let index: u8 = x.into();
        self.vs[usize::from(index)] = nn;
    }

    fn op_7xnn(&mut self, x: u4, nn: u8) {
        let index: u8 = x.into();
        self.vs[usize::from(index)] = self.vs[usize::from(index)] + nn
    }

    fn op_annn(&mut self, nnn: u12) {
        self.index = nnn
    }

    fn op_dxyn(&mut self, x: u4, y: u4, n: u4) {
        let x_index: u8 = x.into();
        let x_index: usize = usize::from(x_index);

        let y_index: u8 = y.into();
        let y_index: usize = usize::from(y_index);

        let mut vx: u8 = self.vs[x_index];
        let mut vy: u8 = self.vs[y_index];

        //starting position of draw should be wrapped
        vx = vx % WIDTH_U8;
        vy = vy % HEIGHT_U8;

        self.vs[0xF] = 0;

        let last_row: u8 = n.into();
        for i in (0..last_row) {
            let sprite_row: u8 = self.ram.read(self.index + i.into());
            for col in (0..8) {
                //Check whether we've hit the right edge of the screen
                if (vx + col >= WIDTH_U8) {
                    break;
                }

                //Grab the ``col``th pixel in sprite row
                let sprite_pixel = (sprite_row >> col) & (0x01);

                let screen_x = usize::from(vx + col);
                let screen_y = usize::from(vy);
                let screen_pixel: bool = self.vram[screen_x][screen_y];
                if (sprite_pixel == 1) {
                    if screen_pixel {
                        self.vram[screen_x][screen_y] = false;
                        self.vs[0xF] = 1;
                    } else {
                        self.vram[screen_x][screen_y] = true;
                    }
                }
            }

            vy += 1;
        }
    }

    pub fn view(&self) -> () {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let pixel = self.vram[x][y];
                if pixel {
                    print!("■");
                } else {
                    print!(" ");
                }
            }
            println!();
        }
    }
}
