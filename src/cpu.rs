use crate::error::*;
use crate::memory::Memory;
use ux::*;
use rand::{thread_rng, Rng};

pub const HEIGHT: usize = 32;
pub const WIDTH: usize = 64;

const HEIGHT_U8: u8 = 32;
const WIDTH_U8: u8 = 64;

#[derive(Debug, Clone)]
pub struct CPU {
    ram: Memory,                   // 4kB of RAM
    pub vram: [[bool; HEIGHT]; WIDTH], //vram containing pixel values, stored in column-major order
    stack: Vec<u16>,               // stack, comprising of 2-byte values
    pc: u12,                       // program counter
    index: u12,                    // index register "I", used to point to addresses in memory
    pub delay : u8,                    // delay timer, decremented at a rate of 60Hz until it reaches 0
    pub beep : u8,                     // sound timer, should emit a beeping sound as long as it's not 0
    pub vs: [u8; 16], // general-purpose registers, labeled V0-VF
}

pub type KeyState = [bool; 16];
pub const NO_KEYS : KeyState = [false; 16];
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

    let mut nib_1_u12: u12 = nib_1.into();
    nib_1_u12 = nib_1_u12 << 8;
    twelve = twelve + nib_1_u12;

    let mut nib_2_u12: u12 = nib_2.into();
    nib_2_u12 = nib_2_u12 << 4;
    twelve = twelve + nib_2_u12;

    twelve = twelve + nib_3.into();

    twelve
}

pub fn nib_to_usize(nib : u4) -> usize {
    let wide : u8 = nib.into();
    wide.into()
}

pub fn upper_nib(byte : u8) -> u4 {
    ((byte & (0xF0)) >> 4).try_into().unwrap()
} 
pub fn lower_nib(byte : u8) -> u4 {
    (byte & (0x0F)).try_into().unwrap()
} 

#[derive(Debug, Clone)]
pub enum Opcode {
    ClearScreen,
    Jump(u12),
    JumpOffset(u12),
    SetReg(u4, u8),
    AddReg(u4, u8),
    SetI(u12),
    Display(u4, u4, u4),
    // Control flow instructions
    SkipEqImm(u4, u8),
    SkipNeqImm(u4, u8),
    SkipEqReg(u4, u4),
    SkipNeqReg(u4, u4),
    // Subroutine call 
    CallSubroutine(u12),
    Return,
    //Arithmetic and logical instructions 
    Set(u4, u4), 
    Or(u4, u4),
    And(u4, u4),
    Xor(u4, u4),
    Add(u4, u4),
    Subtract1(u4, u4), // VX - VY
    Subtract2(u4, u4), // VY - VX
    ShiftR(u4, u4),
    ShiftL(u4, u4),
    // Memory instructions
    Store(u4),
    Load(u4),
    //Timers
    SetRegToDelay(u4),
    SetDelayToReg(u4),
    SetSoundToReg(u4),
    //Input
    SkipIfKey(u4),
    SkipIfNotKey(u4),
    GetKey(u4),
    // Miscellaneous
    Decimal(u4),
    AddToIndex(u4),
    Random(u4, u8),
    Font(u4)
}

impl CPU {
    /* Initializes the CPU by allocating a fresh block of
    memory and setting registers to their initial values*/
    pub fn new() -> Self {
        let ram: Memory = Memory::new();

        CPU {
            ram: ram,
            vram: [[false; HEIGHT]; WIDTH],
            stack: vec![],
            pc: (0x200u16).try_into().unwrap(),
            index: 0x0.into(),
            delay: 0x00,
            beep: 0x00,
            vs: [0; 16],
        }
    }

    /* Returns an immutable reference to the ram for debugging purposes */
    pub fn ram(&self) -> &Memory {
        &self.ram
    }
    /* Loads a program into memory */
    pub fn load_program(&mut self, data: &[u8]) -> () {
        self.ram.load_program(data);
    }

    /* Simulates one CPU cycle, returning an error if decoding fails */
    pub fn step(&mut self) -> Result<()> {
        let instr = self.fetch();
        let opcode = self.try_decode(instr)?;
        self.execute(opcode, NO_KEYS);
        Ok(())
    }

    pub fn decr_delay(&mut self) -> () {
        if self.delay > 0 {
            self.delay -= 1;
        }
    }

    pub fn decr_sound(&mut self) -> () {
        if self.beep > 0 {
            self.beep -= 1;
        }
    }

    pub fn decr_timers(&mut self) -> () {
        self.decr_delay();
        self.decr_sound();
    }
    /* Fetches the current instruction pointed to by the PC. Increments the PC by 2 */
    pub fn fetch(&mut self) -> (u8, u8) {
        let byte_1: u8 = self.ram.read(self.pc);
        let byte_2: u8 = self.ram.read(self.pc + 1.into());

        self.pc = self.pc + 2.into();
        (byte_1, byte_2)
    }

    /* Decodes ``instr``, returning the Opcode it corresponds to */
    pub fn decode(&self, instr: (u8, u8)) -> Opcode {
        self.try_decode(instr)
            .expect("Could not parse {instr:?} to opcode")
    }
    /* Decodes ``instr``, returning the Opcode it corresponds to */
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
            (byte_1 @ 0x30..=0x3F, byte_2) => {
                let NibblePair(_, nib_1) = byte_1.into();
                Opcode::SkipEqImm(nib_1, byte_2)

            }
            // 4XNN
            (byte_1 @ 0x40..=0x4F, byte_2) => {
                let NibblePair(_, nib_1) = byte_1.into();
                Opcode::SkipNeqImm(nib_1, byte_2)

            }
            
            // 5XY0
            (byte_1 @ 0x50..=0x5F, byte_2) if lower_nib(byte_2) == (0u8).try_into().unwrap() => {
                let NibblePair(_, nib_1) = byte_1.into();
                let NibblePair(nib_2, _) = byte_2.into();
                Opcode::SkipEqReg(nib_1, nib_2)
            }

            // 9XY0 
            (byte_1 @ 0x90..=0x9F, byte_2) if lower_nib(byte_2) == (0u8).try_into().unwrap() => {
                let NibblePair(_, nib_1) = byte_1.into();
                let NibblePair(nib_2, _) = byte_2.into();
                Opcode::SkipNeqReg(nib_1, nib_2)
            }

            // 2NNN 
            (byte_1 @ 0x20..=0x2F, byte_2) => {
                let NibblePair(_, nib_1) = byte_1.into();
                let NibblePair(nib_2, nib_3) = byte_2.into();
                Opcode::CallSubroutine(nibtrio_2_u12((nib_1, nib_2, nib_3)))
            }

            // 00EE
            (0x00, 0xEE) => Opcode::Return,

            (byte_1 @ 0x80..=0x8F, byte_2) => {
               self.decode_logarith((byte_1, byte_2)).unwrap()
            }

            (byte_1 @ 0xF0..=0xFF, 0x55) => {
                Opcode::Store(lower_nib(byte_1))
            }

            (byte_1 @ 0xF0..=0xFF, 0x65) => {
                Opcode::Load(lower_nib(byte_1))
            }

            (byte_1 @ 0xF0..=0xFF, 0x33) => {
                Opcode::Decimal(lower_nib(byte_1))
            }

            (byte_1 @ 0xF0..=0xFF, 0x1E) => {
                Opcode::AddToIndex(lower_nib(byte_1))
            }

            (byte_1 @ 0xC0..=0xCF, byte_2) => {
                Opcode::Random(lower_nib(byte_1), byte_2)
            }

            (byte_1 @ 0xF0..=0xFF, 0x29) => {
                Opcode::Font(lower_nib(byte_1))
            }
            (byte_1 @ 0xF0..=0xFF,0x07) => {
                Opcode::SetRegToDelay(lower_nib(byte_1))
            }

            (byte_1 @ 0xF0..=0xFF,0x15) => {
                Opcode::SetDelayToReg(lower_nib(byte_1))
            }

            (byte_1 @ 0xF0..=0xFF,0x18) => {
                Opcode::SetSoundToReg(lower_nib(byte_1))
            }

            (byte_1 @ 0xB0..=0xBF,byte_2) => {
                let NibblePair(_, nib_1) = byte_1.into();
                let NibblePair(nib_2, nib_3) = byte_2.into();
                Opcode::JumpOffset(nibtrio_2_u12((nib_1, nib_2, nib_3)))
            }

            (byte_1 @ 0xE0..=0xEF, 0x9E) => {
                let NibblePair(_, key) = byte_1.into();
                Opcode::SkipIfKey(key)
            }

            (byte_1 @ 0xE0..=0xEF, 0xA1) => {
                let NibblePair(_, key) = byte_1.into();
                Opcode::SkipIfNotKey(key)
            }

            (byte_1 @ 0xF0..=0xFF, 0x0A) => {
                let NibblePair(_, key) = byte_1.into();
                Opcode::GetKey(key)
            }

            _ => {
                return Err(Chip8Error::DecodeError {
                    instr,
                    reason: "No decoding implementation found for this hex range".to_string(),
                })
            }
        };
        Ok(opcode)
    }

    // Decodes logical and arithmetic instructions; assumes that the upper byte of the instruction is in the 
    // range 0x80..0x8F
    pub fn decode_logarith(&self, (byte_1, byte_2): (u8, u8)) -> Result<Opcode>{
        if !(0x80..=0x8F).contains(&byte_1) {
            return Err(Chip8Error::DecodeError {
                instr: (byte_1, byte_2),
                reason : "Upper byte of supposed logical or arithmetic instruction is not within range 0x80..0x8F".to_string(),
            })
        }

        else {
            let x = lower_nib(byte_1);
            let y = upper_nib(byte_2);
            match u8::from(lower_nib(byte_2)) {
                0x0 => Ok(Opcode::Set(x, y)),
                0x1 => Ok(Opcode::Or(x, y)),
                0x2 => Ok(Opcode::And(x, y)),
                0x3 => Ok(Opcode::Xor(x,y)),
                0x4 => Ok(Opcode::Add(x,y)),
                0x5 => Ok(Opcode::Subtract1(x, y)),
                0x7 => Ok(Opcode::Subtract2(x, y)),
                0x6 => Ok(Opcode::ShiftR(x, y)),
                0xE => Ok(Opcode::ShiftL(x, y)),
                _ => {
                    return Err(Chip8Error::DecodeError {
                        instr : (byte_1, byte_2),
                        reason: "No decoding implementation found for this hex range".to_string(),
                    })
                }

            }
        }

    }

    /* Executes the instruction indicated by ``opcode`` */
    pub fn execute(&mut self, opcode: Opcode, keystate : KeyState) -> () {
        match opcode {
            Opcode::ClearScreen => self.op_00e0(),
            Opcode::Jump(addr) => self.op_1nnn(addr),
            Opcode::SetReg(reg, value) => self.op_6xnn(reg, value),
            Opcode::AddReg(reg, value) => self.op_7xnn(reg, value),
            Opcode::SetI(addr) => self.op_annn(addr),
            Opcode::Display(x, y, n) => self.op_dxyn(x, y, n),
            Opcode::SkipEqImm(x, nn) => self.op_3xnn(x, nn),
            Opcode::SkipNeqImm(x, nn) => self.op_4xnn(x, nn),
            Opcode::SkipEqReg(x, y) => self.op_5xy0(x, y),
            Opcode::SkipNeqReg(x, y) => self.op_9xy0(x, y),
            Opcode::CallSubroutine(nnn) => self.op_2nnn(nnn),
            Opcode::Return => self.op_00ee(),
            Opcode::Set(x, y) => self.op_8xy0(x, y),
            Opcode::Or(x, y) => self.op_8xy1(x, y),
            Opcode::And(x, y) => self.op_8xy2(x, y),
            Opcode::Xor(x, y) => self.op_8xy3(x, y),
            Opcode::Add(x, y) => self.op_8xy4(x, y),
            Opcode::Subtract1(x, y) => self.op_8xy5(x, y),
            Opcode::Subtract2(x, y) => self.op_8xy7(x, y),
            Opcode::ShiftR(x, y) => self.op_8xy6_modern(x, y),
            Opcode::ShiftL(x, y) => self.op_8xye_modern(x, y),
            Opcode::Store(x) => self.op_fx55_modern(x),
            Opcode::Load(x) => self.op_fx65_modern(x),
            Opcode::Decimal(x) => self.op_fx33(x),
            Opcode::AddToIndex(x) => self.op_fx1e(x),
            Opcode::Random(x, nn) => self.op_cxnn(x, nn),
            Opcode::Font(x) => self.op_fx29(x),
            Opcode::SetRegToDelay(x) => self.op_fx07(x),
            Opcode::SetDelayToReg(x) => self.op_fx15(x),
            Opcode::SetSoundToReg(x) => self.op_fx18(x),
            Opcode::JumpOffset(nnn) => self.op_bnnn_modern(nnn),
            Opcode::SkipIfKey(x) => self.op_ex9e(x, keystate),
            Opcode::SkipIfNotKey(x) => self.op_exa1(x, keystate),
            Opcode::GetKey(x) => self.op_fx0a(x, keystate)
            
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
        self.vs[usize::from(index)] = self.vs[usize::from(index)].wrapping_add(nn)
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
                if vx + col >= WIDTH_U8 {
                    break;
                }

                //Grab the ``col``th pixel in sprite row
                let sprite_pixel = (sprite_row >> (7 - col)) & (0x01);

                let screen_x = usize::from(vx + col);
                let screen_y = usize::from(vy);
                let screen_pixel: bool = self.vram[screen_x][screen_y];
                if sprite_pixel == 1 {
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



    fn skip_opcode(&mut self) -> () {
        self.pc = self.pc + 2.into();
    }
    fn op_3xnn(&mut self, x  : u4, nn : u8) -> () {
        let index : usize = nib_to_usize(x);
        if (self.vs[index] == nn) {
            self.skip_opcode();
        }
    }

    fn op_4xnn(&mut self, x  : u4, nn : u8) -> () {
        let index: usize = nib_to_usize(x);
        if (self.vs[index] != nn) {
            self.skip_opcode();
        }
    }

    fn op_5xy0(&mut self, x : u4, y : u4) -> () {
        let index_x : usize = nib_to_usize(x);
        let index_y : usize = nib_to_usize(y);

        if (self.vs[index_x] == self.vs[index_y]) {
            self.skip_opcode();
        }


    }

    fn op_9xy0(&mut self, x : u4, y : u4) -> () {
        let index_x : usize = nib_to_usize(x);
        let index_y : usize = nib_to_usize(y);

        if (self.vs[index_x] != self.vs[index_y]) {
            self.skip_opcode();
        }
    }

    fn op_2nnn(&mut self, nnn : u12) -> () {
        self.stack.push(self.pc.into());
        self.pc = nnn;
    }

    fn op_00ee(&mut self) -> () {
        let return_addr = self.stack.pop().unwrap();
        self.pc = return_addr.try_into().unwrap();
    }

    fn load_from(&self, reg : u4) -> u8 {
        self.vs[nib_to_usize(reg)]
    }

    fn save_to(&mut self, reg : u4, val : u8) -> () {
        self.vs[nib_to_usize(reg)] = val
    }

    fn op_8xy0(&mut self, x : u4, y : u4) -> () {
        let vy = self.load_from(y);
        self.save_to(x, vy)
    }

    // Bitwise OR
    fn op_8xy1(&mut self, x : u4, y : u4) -> () { 
        let vx = self.load_from(x);
        let vy = self.load_from(y);
        self.save_to(x, vx | vy)
    }
    // Bitwise AND
    fn op_8xy2(&mut self, x : u4, y : u4) -> () { 
        let vx = self.load_from(x);
        let vy = self.load_from(y);
        self.save_to(x, vx & vy)
    }
    // Bitwise XOR
    fn op_8xy3(&mut self, x : u4, y : u4) -> () { 
        let vx = self.load_from(x);
        let vy = self.load_from(y);
        self.save_to(x, vx ^ vy)
    }
    // Add
    fn op_8xy4(&mut self, x : u4, y : u4) -> () { 
        let vx = self.load_from(x);
        let vy = self.load_from(y);
        let (add, carry) =  vx.checked_add(vy).map_or_else(
        || {(vx.wrapping_add(vy), true)}, 
        |v| {(v, false)}
        );

        if carry {
            self.vs[0xF] = 0x1;
        }
        else {
            self.vs[0xF] = 0x0;
        }

        self.save_to(x, add);
    }

    // Subtract (Variant 1) : VX - VY
    fn op_8xy5(&mut self, x : u4, y : u4) -> () { 
        let vx = self.load_from(x);
        let vy = self.load_from(y);
        let (sub, carry) =  vx.checked_sub(vy).map_or_else(
        || {(vx.wrapping_sub(vy), true)}, 
        |v| {(v, false)}
        );

        if carry {
            self.vs[0xF] = 0x0;
        }
        else {
            self.vs[0xF] = 0x1;
        }

        self.save_to(x, sub);
    }
    // Subtract (Variant 2) : VY - VX
    fn op_8xy7(&mut self, x : u4, y : u4) -> () {
        let vx = self.load_from(x);
        let vy = self.load_from(y);
        let (sub, carry) =  vy.checked_sub(vx).map_or_else(
        || {(vy.wrapping_sub(vx), true)}, 
        |v| {(v, false)}
        );

        if carry {
            self.vs[0xF] = 0x0;
        }
        else {
            self.vs[0xF] = 0x1;
        }

        self.save_to(x, sub);
    }
    // Returns the last bit of a byte
    fn last_bit(&self, byte : u8) -> u8 {
        byte & (0x01)
    }
    // Returns first bit of a byte 
    fn first_bit(&self, byte : u8) -> u8 {
        byte & (0x08) >> 3
    }

    // Right shift (logical) 
    fn op_8xy6_modern(&mut self, x : u4, _y : u4) -> () {
        let vx = self.load_from(x);
        self.vs[0xF] = self.last_bit(vx);

        self.save_to(x, vx >> 1);

    }

    // Left shift (logical) 
    fn op_8xye_modern(&mut self, x : u4, _y : u4) -> () {
        let vx = self.load_from(x);
        self.vs[0xF] = self.first_bit(vx);

        self.save_to(x, vx << 1);
    }

    // Store
    fn op_fx55_modern(&mut self, x : u4) -> () {
        let index : u12 = self.index;
        let last_reg : u8 = x.into();
        for i in (0..=last_reg) {
            let val = self.load_from(i.try_into().unwrap());
            self.ram.write(index + i.into(), val)
        }
    }

    // Load
    fn op_fx65_modern(&mut self, x : u4) -> () {
        let index : u12 = self.index;
        let last_reg : u8 = x.into();
        for i in (0..=last_reg) {
            let val = self.ram.read(index + i.into());
            self.save_to(i.try_into().unwrap(), val);
        }
    }


    // Decimal conversion
    fn op_fx33(&mut self, x : u4) -> () {
        let vx = self.load_from(x);
        let first_digit = vx/100;
        let second_digit = (vx % 100)/10;
        let third_digit = vx % 10;

        let index = self.index;
        self.ram.write(index, first_digit);
        self.ram.write(index + 1.into(), second_digit);
        self.ram.write(index + 2.into(), third_digit);
    }

    fn op_fx1e(&mut self, x : u4) -> () {
        let vx = self.load_from(x);
        self.index = self.index + vx.into();
        //Note: Some interpreters would set the carry flag if the index register overflow from 0xFFF to 0x1000+ (outside of addressable range),
        // consider adding an option to do
    }

    fn op_cxnn(&mut self, x : u4, nn : u8) -> () {
        let mut rng = thread_rng();
        let rand : u8 = rng.gen();
        let result = rand & nn;
        self.save_to(x, result)
    }

    fn op_fx29(&mut self, x : u4) -> () {
        let vx = self.load_from(x);
        let sprite_addr = (vx & (0x0F)) * 0x5;
        self.index = sprite_addr.into();
    }

    fn op_fx07(&mut self, x : u4) -> () {
        self.save_to(x, self.delay)
    }

    fn op_fx15(&mut self, x : u4) -> () {
        self.delay = self.load_from(x)
    }

    fn op_fx18(&mut self, x : u4) -> () {
        self.beep = self.load_from(x)
    }

    fn op_bnnn_modern(&mut self, nnn : u12) -> () {
        let nnn_16 : u16 = nnn.into(); 
        let x : u4 = ((nnn_16 & (0xF00)) >> 8).try_into().unwrap();

        let vx = self.load_from(x);
        let addr = nnn + vx.into();
        self.pc = addr
    }

    fn op_ex9e(&mut self, x : u4, keystate : KeyState) -> () {
        let index = nib_to_usize(x);
        let vx : usize = self.vs[index].into();
        if keystate[vx] {
            self.pc = self.pc + 2.into();
        }
    }

    fn op_exa1(&mut self, x : u4, keystate : KeyState) -> () {
        let index = nib_to_usize(x);
        let vx : usize = self.vs[index].into();
        if !keystate[vx] {
            self.pc = self.pc + 2.into();
        }
    }
    fn op_fx0a(&mut self, x : u4, keystate : KeyState) -> () {
        let index = nib_to_usize(x);
        let mut key_press : Option<u8> = None;

        for i in (0u8..=0xFu8) {
            let index : usize = i.into();
            if keystate[index] {
                key_press = Some(i);
                break
            }
        }
        
        if key_press.is_some() {
            self.vs[index] = key_press.unwrap();
            return;
        }

        self.pc = self.pc - 2.into()

    }
    pub fn view(&self) -> () {
        print!("   ");
        for _ in 0..WIDTH {
            print!("-");
        }
        println!();
        for y in 0..HEIGHT {
            print!("{:02}", y);
            print!("|");
            for x in 0..WIDTH {
                let pixel = self.vram[x][y];
                if pixel {
                    print!("■");
                } else {
                    print!(" ");
                }
            }
            println!("|");
        }

        print!("   ");
        for _ in 0..WIDTH {
            print!("-");
        }
        println!();
    }

    pub fn program_counter(&self) -> u16 {
        u12::into(self.pc)
    }
}
