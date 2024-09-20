# Chip8-Crab

A Chip-8 emulator written in Rust


## Roadmap

- [x] Implement the fetch, decode, and execute loop 
- [x] Connect to graphics library 
- [x] Add keyboard input
- [ ] Add tests

## Subgoals

- [x] REPL: Add ability to load ROMS from binary files
- [x] REPL: Add visual display to show the current state of the emulator's vram

## Running the emulator

To run the emulator, use the following command:

```bash
cargo run --bin main
```

You will then be asked to provide a path to the ROM to load.
If an error occurs, be sure that [SDL2](https://github.com/Rust-SDL2/rust-sdl2?tab=readme-ov-file#requirements) is installed on your system.


## Running the REPL debugger
 
To run the , use the following command:

```bash
cargo run --bin repl
```

Usage examples:

```
load roms/pong.ch8 # loads a file located at roms/pong.ch8 to a new CPU
l roms/pong.ch8    # same as above

run              # runs the CPU until it crashes, the program ends, or a breakpoint is hit
r                # same as above

step             # steps one instruction
s                # same as above

s 100            # steps 0x100 instructions
s 0x100          # same as above 


debug            # prints the current state of the CPU
d                # same as above

breakpoint 0x200 # sets a breakpoint when pc @ 0x200
b 0x200          # same as above

execute 0x00e0   # executes the command 0x00e0 on the CPU 
e 0x00e0         # same as above

view             # prints the current state of the vram
v                # same as above

memory          # prints the current state of the memory around the program counter
m               # same as above
mem             # same as above

memory 0x2FF    # prints the current state of the memory around the address 0x2FF
m 0x2FF         # same as above

```

# References

- [Guide to making a CHIP-8 emulator](https://tobiasvl.github.io/blog/write-a-chip-8-emulator/#display)
- [Writing a CHIP-8 emulator](https://austinmorlan.com/posts/chip8_emulator/)
