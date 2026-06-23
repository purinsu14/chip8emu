# CHIP8 Emulator in Rust

A CHIP-8 emulator written in Rust.

## Features

* Fully functional CHIP-8 emulator
* Compatible with a wide range of CHIP-8 ROMs
* Adjustable emulation speed
* Cross-platform Rust implementation
* Pre-built Linux binaries available

## Installation

### With Rust

Clone the repository and install:

```bash
git clone https://github.com/purinsu14/chip8emu.git
cd chip8emu
cargo install --path .
```

Run a ROM:

```bash
chip8 <rom> [cycles]
```

Or run directly with Cargo:

```bash
cargo run -- <rom> [cycles]
```

### With Pre-built Binary (Linux)

Download the latest release binary from:

https://github.com/purinsu14/chip8emu/releases

Make it executable and run it:

```bash
chmod +x <filename>
./<filename> <rom> [cycles]
```

## Usage

```bash
chip8 <rom> [cycles]
```

* `rom` is the CHIP-8 ROM to load.
* `cycles` controls emulation speed and is optional.
* Default `cycles` value is `10`.

Some ROMs may require a higher or lower cycle count for proper gameplay speed.

## Default Keypad Mapping

```text
1 2 3 4  →  1 2 3 C
Q W E R  →  4 5 6 D
A S D F  →  7 8 9 E
Z X C V  →  A 0 B F
```

## ROMs

Compatible CHIP-8 ROMs can be found here:

https://github.com/kripod/chip8-roms

## Contributing

Contributions are welcome. Feel free to open an issue or submit a pull request if you find a bug or have ideas for improvements.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

---

Made by [purinsu14](https://github.com/purinsu14)
