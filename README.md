# Chip8-Crab

A Chip-8 emulator written in Rust


## Roadmap

- [ ] Implement the fetch, decode, and execute loop 
- [ ] Connect to graphics library 
- [ ] Add keyboard input
- [ ] Add tests

## Subgoals

- [x] REPL: Add ability to load ROMS from binary files
- [x] REPL: Add visual display to show the current state of the emulator's vram

## Running the emulator

To run the emulator, use the following command:

```bash
cargo run --bin repl
```

commands:

```
load <filename> - loads a file into the cpu but does not execute it
run - steps on instructions in the current cpu until it crashes
step - steps one instruction (fetch -> decode -> execute)
debug - views debug information (registers rn)
execute <0xNNNN> - executes a hex command on the CPU 
view - prints the vram
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
