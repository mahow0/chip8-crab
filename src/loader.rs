use std::fs;
use crate::error::*;
use crate::cpu::CPU;

pub fn load_rom(filename: &str) -> Result<CPU> {
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

    let mut cpu = CPU::new();

    for instr in rom_tuples {
        let decode = cpu.try_decode(instr)?;
        cpu.execute(decode);
    }

    Ok(cpu)
}
