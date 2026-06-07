## CHIP8 in Rust

A CHIP-8 emulator written in Rust.

Pre-built binary for Linux available on the [releases page](https://github.com/purinsu14/chip8emu/releases).

## Install and run (with Rust)

Extract the source code, then:
```
cargo install --path .
chip8 <rom> [cycles]
```
or
```
cargo run -- <rom> [cycles]
```

## Install with binary (no Rust needed)
Download the binary, open terminal on download path, then:
```
chmod +x <filename>
./<filename> <rom> [cycles]
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
Compatible ROMs can be found [here](https://github.com/kripod/chip8-roms).

---
*Made by [purinsu14](https://github.com/purinsu14)*
