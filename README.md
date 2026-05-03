### CHIP8 in Rust

A CHIP-8 emulator written in Rust.

## Install

Extract the source code, then:
```
cargo install --path
chip8 <rom> [cycles]
```
or
```
cargo run -- <rom> [cycles]
```
Example:
```
chip8 IBM Logo.ch8 7
```
Additional note:
`cycles` controls emulation speed (default: 10). Adjust per ROM if too fast or slow.

## Default Keypad

```
1 2 3 4  →  1 2 3 C
Q W E R  →  4 5 6 D
A S D F  →  7 8 9 E
Z X C V  →  A 0 B F
```

## ROMs
Compatible ROMs can be found at https://github.com/kripod/chip8-roms
