use crate::cpu::{Opcode, CPU, NO_KEYS};
use crate::error::*;
use std::fs;

/// Accepts a binary file and returns a vector of bytes.
fn load_bytes(filename: &str) -> Result<Vec<u8>> {
    fs::read(filename).map_err(|err| Chip8Error::ROMLoaderError {
        reason: err.to_string(),
    })
}

/// Accepts a binary file where every 2 bytes corresponds to a
/// chip8 instruction. Returns a vector of decoded opcodes.
fn load_opcodes(filename: &str) -> Result<Vec<Opcode>> {
    let rom = load_bytes(filename)?;
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

    let mut opcodes = Vec::new();
    let cpu = CPU::new();
    for instr in rom_tuples {
        let opcode = cpu.try_decode(instr)?;
        opcodes.push(opcode);
    }

    Ok(opcodes)
}

/// Accepts a binary file corresponding to a series of
/// raw u16 bytes to put into program memory. Returns a CPU with the ROM loaded.
pub fn load_program(filename: &str) -> Result<CPU> {
    let bytes = load_bytes(filename)?;
    let mut cpu = CPU::new();
    cpu.load_program(&bytes);
    Ok(cpu)
}

/// Accepts a binary file where every 2 bytes corresponds to a
/// chip8 instruction. Executes each instruction in the CPU.
pub fn run(filename: &str, cpu: &mut CPU) -> Result<()> {
    for opcode in load_opcodes(filename)? {
        cpu.execute(opcode, NO_KEYS);
    }
    Ok(())
}
