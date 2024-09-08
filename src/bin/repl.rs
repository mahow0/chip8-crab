use chip8_crab::cpu::*;
use chip8_crab::error::*;
use chip8_crab::loader;
use regex::Regex;

fn parse_command(command: &str) -> Result<(Command, String)> {
    let cap = Regex::new(r"(\w+)(.*)").unwrap().captures(command).unwrap();
    let command = cap.get(1).unwrap().as_str();

    let rest = cap.get(2).unwrap().as_str().to_string();

    match command {
        "l" | "load" | "lo" | "loa" => Ok((Command::Load, rest)),
        "r" | "run" | "ru" => Ok((Command::Run, rest)),
        "s" | "step" | "ste" | "st" => Ok((Command::Step, rest)),
        "d" | "de" | "deb" | "debu" | "debug" => Ok((Command::Debug, rest)),
        "q" | "qu" | "qui" | "quit" | "exit" => Ok((Command::Quit, rest)),
        "e" | "ex" | "exe" | "exec" | "execu" | "execut" | "execute" => {
            Ok((Command::Execute, rest))
        }
        "v" | "vi" | "vie" | "view" => Ok((Command::View, rest)),
        _ => Err(Chip8Error::CommandParseError(command.to_string())),
    }
}

fn parse_instruction(instruction: &str) -> Result<u16> {
    let re = Regex::new(r"(0x)?([0-9A-Fa-f]{4,6})").unwrap();
    let cap = re.captures(instruction).expect("Capture failed");
    let hex = cap.get(2);
    if hex.is_none() {
        return Err(Chip8Error::InstructionParseError(instruction.to_string()));
    }
    let hex = hex.unwrap().as_str();
    let hex = u16::from_str_radix(hex, 16);
    if hex.is_err() {
        return Err(Chip8Error::NumericalConversionError(
            hex.unwrap_err().to_string(),
        ));
    }
    let hex = hex.unwrap();
    Ok(hex)
}

#[derive(Debug, PartialEq)]
enum Command {
    /// Load a ROM into the CPU but do not yet execute it
    Load,
    /// Load the instructions from a ROM and execute them
    Run,
    Step,
    Quit,
    /// Print the state of the CPU (registers)
    Debug,
    /// Execute a single instruction in hex format (0xNNNN or NNNN)
    Execute,
    /// View the current vram
    View,

}

fn main() {
    let mut cpu = CPU::new();
    loop {
        // TODO: figure out how to print without new line
        println!("Enter a command: ");

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        let (command, rest) = parse_command(&input).expect("execution failed:\n");

        match command {
            Command::Load => {
                let filename = rest.trim();
                cpu = loader::load_rom(&filename).expect("rom load failed:\n");
            }

            Command::Run => {
                let filename = rest.trim();
                loader::run(&filename, &mut cpu).expect("run failed:\n");
            }

            Command::Execute => {
                println!("Executing: {}", rest);
                let opcode = parse_instruction(&rest).expect("execution failed:\n");
                let (a, b) = (opcode >> 8, opcode & 0x00FF);
                let (a, b) = (a as u8, b as u8);
                let decoded_opcode = cpu.decode((a, b));
                cpu.execute(decoded_opcode);
            }

            Command::Debug => {
                println!("Debugging");
                for i in 0..16 {
                    println!("V{:X}: {}", i, cpu.vs[i]);
                }
            }

            Command::View => {
                cpu.view();
            }

            _ => {}
        }
    }
}

#[cfg(test)]
pub mod repl_tests {
    use super::*;
    #[test]
    pub fn test_parse_command() {
        let (command, rest) = parse_command("view test").unwrap();
        assert_eq!(command, Command::View);
        assert_eq!(rest, " test");

        let (command, rest) = parse_command("vie test").unwrap();
        assert_eq!(command, Command::View);
        assert_eq!(rest, " test");

        let (command, rest) = parse_command("vi test").unwrap();
        assert_eq!(command, Command::View);
        assert_eq!(rest, " test");

        let (command, rest) = parse_command("v test").unwrap();
        assert_eq!(command, Command::View);
        assert_eq!(rest, " test");

        let (command, rest) = parse_command("load test").unwrap();
        assert_eq!(command, Command::Load);
        assert_eq!(rest, " test");

        let (command, rest) = parse_command("l test").unwrap();
        assert_eq!(command, Command::Load);
        assert_eq!(rest, " test");

        let (command, rest) = parse_command("lo test").unwrap();
        assert_eq!(command, Command::Load);
        assert_eq!(rest, " test");

        let (command, rest) = parse_command("loa test").unwrap();
        assert_eq!(command, Command::Load);
        assert_eq!(rest, " test");

        let (command, rest) = parse_command("run test").unwrap();
        assert_eq!(command, Command::Run);
        assert_eq!(rest, " test");

        let (command, rest) = parse_command("r test").unwrap();
        assert_eq!(command, Command::Run);
        assert_eq!(rest, " test");

        let (command, rest) = parse_command("ru test").unwrap();
        assert_eq!(command, Command::Run);
        assert_eq!(rest, " test");

        let (command, rest) = parse_command("step test").unwrap();
        assert_eq!(command, Command::Step);
        assert_eq!(rest, " test");

        let (command, rest) = parse_command("ste test").unwrap();
        assert_eq!(command, Command::Step);
        assert_eq!(rest, " test");

        let (command, rest) = parse_command("st test").unwrap();
        assert_eq!(command, Command::Step);
        assert_eq!(rest, " test");

        let (command, rest) = parse_command("s test").unwrap();
        assert_eq!(command, Command::Step);
        assert_eq!(rest, " test");

        let (command, rest) = parse_command("quit test").unwrap();
        assert_eq!(command, Command::Quit);
        assert_eq!(rest, " test");

        let (command, rest) = parse_command("qui test").unwrap();
        assert_eq!(command, Command::Quit);
        assert_eq!(rest, " test");

        let (command, rest) = parse_command("qu test").unwrap();
        assert_eq!(command, Command::Quit);
        assert_eq!(rest, " test");

        let (command, rest) = parse_command("q test").unwrap();
        assert_eq!(command, Command::Quit);
        assert_eq!(rest, " test");

        let (command, rest) = parse_command("debug  test").unwrap();
        assert_eq!(command, Command::Debug);
        assert_eq!(rest, "  test");

        let (command, rest) = parse_command("debu  test").unwrap();
        assert_eq!(command, Command::Debug);
        assert_eq!(rest, "  test");

        let (command, rest) = parse_command("deb  test").unwrap();
        assert_eq!(command, Command::Debug);
        assert_eq!(rest, "  test");

        let (command, rest) = parse_command("de  test").unwrap();
        assert_eq!(command, Command::Debug);
        assert_eq!(rest, "  test");

        let (command, rest) = parse_command("d  test").unwrap();
        assert_eq!(command, Command::Debug);
        assert_eq!(rest, "  test");

        let (command, rest) = parse_command("execute test").unwrap();
        assert_eq!(command, Command::Execute);
        assert_eq!(rest, " test");

        let (command, rest) = parse_command("execut test").unwrap();
        assert_eq!(command, Command::Execute);
        assert_eq!(rest, " test");

        let (command, rest) = parse_command("execu test").unwrap();
        assert_eq!(command, Command::Execute);
        assert_eq!(rest, " test");

        let (command, rest) = parse_command("exec test").unwrap();
        assert_eq!(command, Command::Execute);
        assert_eq!(rest, " test");

        let (command, rest) = parse_command("exe test").unwrap();
        assert_eq!(command, Command::Execute);
        assert_eq!(rest, " test");

        let (command, rest) = parse_command("ex test").unwrap();
        assert_eq!(command, Command::Execute);
        assert_eq!(rest, " test");

        let (command, rest) = parse_command("e test").unwrap();
        assert_eq!(command, Command::Execute);
        assert_eq!(rest, " test");
    }
}
