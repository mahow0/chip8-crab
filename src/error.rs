use thiserror::Error;
pub type Result<T> = std::result::Result<T, Chip8Error>;

#[derive(Error, Debug)]
pub enum Chip8Error {
    #[error("Could not load ROM: {reason:?}")]
    ROMLoaderError { reason: String },
    #[error("Could not decode {instr:?} because {reason:?}")]
    DecodeError { 
        instr: (u8, u8),
        reason: String 
    },
    #[error("Could not parse command: {0}")]
    CommandParseError(String),
    #[error("Could not parse opcode: {0}")]
    OpcodeParseError(String),
    #[error("Could not convert number: {0}")]
    NumericalConversionError(String),
}
