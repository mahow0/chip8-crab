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

# References

- [Guide to making a CHIP-8 emulator](https://tobiasvl.github.io/blog/write-a-chip-8-emulator/#display)
- [Writing a CHIP-8 emulator](https://austinmorlan.com/posts/chip8_emulator/)
