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
        "b" | "br" | "bre" | "brea" | "break" | "breakpoint" => Ok((Command::Breakpoint, rest)),
        _ => Err(Chip8Error::CommandParseError(command.to_string())),
    }
}

fn parse_hex(input: &str) -> Result<u16> {
    let re = Regex::new(r"(0x)?([0-9A-Fa-f]{1,6})").unwrap();
    let cap = re.captures(input).expect("Capture failed");
    let hex = cap.get(2);
    if hex.is_none() {
        return Err(Chip8Error::InstructionParseError(input.to_string()));
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
    /// Load the instructions stored in CPU memory
    Run,
    Step,
    Quit,
    /// Print the state of the CPU (registers)
    Debug,
    /// Execute a single instruction in hex format (0xNNNN or NNNN)
    Execute,
    /// View the current vram
    View,
    /// Toggles a breakpoint on the pc of the CPU
    /// If a breakpoint is hit, the CPU will pause execution and return to the REPL
    Breakpoint,
}

fn main() {
    let mut cpu = CPU::new();
    let mut breakpoints = Vec::new();
    loop {
        // TODO: figure out how to print without new line
        println!("Enter a command: ");

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        let result = parse_command(&input);
        if result.is_err() {
            println!("Could not parse command: {}", input);
            println!("{:?}", result);
            continue;
        }
        let (command, rest) = result.unwrap();

        match command {
            Command::Load => {
                let filename = rest.trim();
                let new_cpu = loader::load_program(&filename);
                if new_cpu.is_err() {
                    println!("Could not load program: {}", filename);
                    println!("{:?}", new_cpu);
                    continue
                }
                cpu = new_cpu.unwrap();
            }

            Command::Run => {
                loop {
                    if breakpoints.contains(&cpu.program_counter()) {
                        println!("Breakpoint hit at: {:#X}", cpu.program_counter());
                        break;
                    }
                    if let Err(err) = cpu.step() {
                        cpu.view();
                        println!("Error: {}", err);
                        break;
                    }
                }
            },

            Command::Execute => {
                println!("Executing: {}", rest);
                let opcode = parse_hex(&rest);
                if opcode.is_err() {
                    println!("Could not parse opcode: {}", rest);
                    println!("{:?}", opcode);
                    continue;
                }
                let opcode = opcode.unwrap();
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
                println!("PC: {:#X}", cpu.program_counter());
            }

            Command::View => {
                cpu.view();
            }

            Command::Step => {
                let mut steps = 1;
                if rest.trim().len() > 0 {
                    let result = parse_hex(&rest);
                    if result.is_err() {
                        println!("Could not parse number of steps: {}", rest);
                        println!("{:?}", steps);
                        continue;
                    }
                    steps = result.unwrap();
                }
                for _ in 0..steps {
                    let result = cpu.step();
                    if result.is_err() {
                        println!("Error: {}", result.unwrap_err());
                        cpu.view();
                        break
                    }
                }

            }

            Command::Breakpoint => {
                let addr = parse_hex(&rest);
                if addr.is_err() {
                    println!("Could not parse breakpoint: {}", rest);
                    println!("{:?}", addr);
                    continue;
                } 
                let addr = addr.unwrap();

                if breakpoints.contains(&addr) {
                    println!("Removing breakpoint when the pc is {:#X}", addr);
                    breakpoints.retain(|&x| x != addr);
                } else {
                    println!("Adding breakpoint when the pc is {:#X}", addr);
                }

                breakpoints.push(addr);
            }

            _ => {}
        }
    }
}

#[cfg(test)]
pub mod repl_tests {
    use super::*;
    #[test]
    pub fn test_parse_view_command() {
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
    }

    #[test]
    pub fn test_parse_load_command() {
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
    }

    #[test]
    pub fn test_parse_run_command() {
        let (command, rest) = parse_command("run test").unwrap();
        assert_eq!(command, Command::Run);
        assert_eq!(rest, " test");

        let (command, rest) = parse_command("r test").unwrap();
        assert_eq!(command, Command::Run);
        assert_eq!(rest, " test");

        let (command, rest) = parse_command("ru test").unwrap();
        assert_eq!(command, Command::Run);
        assert_eq!(rest, " test");
    }

    #[test]
    pub fn test_parse_step_command() {
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
    }

    #[test]
    pub fn test_parse_quit_command() {
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
    }

    #[test]
    pub fn test_parse_debug_command() {
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
    }

    #[test]
    pub fn test_parse_execute_command() {
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

    #[test]
    pub fn test_parse_breakpoint_command() {
        let (command, rest) = parse_command("breakpoint test").unwrap();
        assert_eq!(command, Command::Breakpoint);
        assert_eq!(rest, " test");

        let (command, rest) = parse_command("break test").unwrap();
        assert_eq!(command, Command::Breakpoint);
        assert_eq!(rest, " test");

        let (command, rest) = parse_command("brea test").unwrap();
        assert_eq!(command, Command::Breakpoint);
        assert_eq!(rest, " test");

        let (command, rest) = parse_command("bre test").unwrap();
        assert_eq!(command, Command::Breakpoint);
        assert_eq!(rest, " test");

        let (command, rest) = parse_command("br test").unwrap();
        assert_eq!(command, Command::Breakpoint);
        assert_eq!(rest, " test");

        let (command, rest) = parse_command("b test").unwrap();
        assert_eq!(command, Command::Breakpoint);
        assert_eq!(rest, " test");
    }
}
