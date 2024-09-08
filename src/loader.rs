use std::fs;
use crate::error::*;
use crate::cpu::{CPU, Opcode};

/// Accepts a binary file where every 2 bytes corresponds to a 
/// chip8 instruction. Returns a vector of decoded opcodes.
fn load_opcodes(filename: &str) -> Result<Vec<Opcode>> {
    let rom = fs::read(filename).map_err(|err| Chip8Error::ROMLoaderError {
        reason: err.to_string(),
    })?;

    if rom.len() % 2 != 0 {
        return Err(Chip8Error::ROMLoaderError {
            reason: "ROM could not be parsed into groups of (u8, u8)".to_string(),
        });
    }

    let mut rom_tuples = Vec::new();
    for i in (0..rom.len()).step_by(2) {
        let val = (rom[i], rom[i + 1]);
        rom_tuples.push(val);
    }

    let mut opcodes =  Vec::new();
    let cpu = CPU::new();
    for instr in rom_tuples {
        let opcode = cpu.try_decode(instr)?;
        opcodes.push(opcode);
    }

    Ok(opcodes)
}

/// Accepts a binary file where every 2 bytes corresponds to a
/// chip8 instruction. Returns a CPU with the ROM loaded.
pub fn load_rom(filename: &str) -> Result<CPU> {
    let _ = load_opcodes(filename)?;

    unimplemented!("We'll need to return the CPU after loading the ROM, but ROM doesn't exist yet.");
}

/// Accepts a binary file where every 2 bytes corresponds to a
/// chip8 instruction. Executes each instruction in the CPU.
pub fn run(filename: &str, cpu : &mut CPU) -> Result<()> {
    for opcode in load_opcodes(filename)? {
        cpu.execute(opcode);
    }
    Ok(())
}
