use minifb::{Key, Window, WindowOptions};
use rand::random;
use std::io::Write;
use std::time::{Duration, Instant};

const WIDTH: usize = 64;
const HEIGHT: usize = 32;

struct CHIP8 {
    mem: [u8; 4096],
    v: [u8; 16],
    i: u16,
    pc: u16,
    sp: u8,
    stack: [u16; 16],
    disp: [u64; 32],
    delay: u8,
    sound: u8,
    keys: [bool; 16],
}

impl CHIP8 {
    fn new() -> Self {
        //self instructions
        let fonts = [
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80, // F
        ];
        let mut mem = [0u8; 4096];
        mem[0..80].copy_from_slice(&fonts); // copy font data to mem
        Self {
            mem,
            v: [0; 16],
            i: 0,
            pc: 0x200, //starts at 0x200
            sp: 0,
            stack: [0; 16],
            disp: [0; 32],
            delay: 0,
            sound: 0,
            keys: [false; 16],
        }
    }

    fn cycle(&mut self) {
        //cycle instructions

        // check pc bounds
        if self.pc as usize + 1 >= self.mem.len() {
            println!("PC out of bounds: {:#x}", self.pc);
            return;
        }

        // fetch: read 2 bytes from memory at PC, combine them into one u16 opcode, then advance PC by 2.
        let byte1 = self.mem[self.pc as usize] as u16;
        let byte2 = self.mem[(self.pc + 1) as usize] as u16;
        let opcode = (byte1 << 8) + byte2;
        self.pc += 2;

        // decode: execute the opcode
        match (opcode & 0xF000) >> 12 {
            0 => match opcode & 0x000F {
                0 => self.disp = [0; 32],
                0xE => {
                    self.pc = self.stack[self.sp as usize];
                    self.sp -= 1;
                }
                _ => println!("Unknown opcode: {:#04x}", opcode),
            },
            1 => self.pc = opcode & 0x0FFF,
            2 => {
                self.sp += 1;
                self.stack[self.sp as usize] = self.pc;
                self.pc = opcode & 0x0FFF;
            }
            3 => {
                let x = ((opcode & 0x0F00) >> 8) as usize;
                if self.v[x] == (opcode & 0x00FF) as u8 {
                    self.pc += 2;
                }
            }
            4 => {
                let x = ((opcode & 0x0F00) >> 8) as usize;
                if self.v[x] != (opcode & 0x00FF) as u8 {
                    self.pc += 2;
                }
            }
            5 => {
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let y = ((opcode & 0x00F0) >> 4) as usize;
                if self.v[x] == self.v[y] {
                    self.pc += 2;
                }
            }
            6 => {
                let x = ((opcode & 0x0F00) >> 8) as usize;
                self.v[x] = (opcode & 0x00FF) as u8;
            }
            7 => {
                let x = ((opcode & 0x0F00) >> 8) as usize;
                self.v[x] = self.v[x].wrapping_add((opcode & 0x00FF) as u8);
            }
            8 => {
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let y = ((opcode & 0x00F0) >> 4) as usize;
                match opcode & 0x000F {
                    0 => self.v[x] = self.v[y],
                    1 => self.v[x] |= self.v[y],
                    2 => self.v[x] &= self.v[y],
                    3 => self.v[x] ^= self.v[y],
                    4 => {
                        let sum = self.v[x] as u16 + self.v[y] as u16;
                        self.v[0xF] = if sum > 255 { 1 } else { 0 };
                        self.v[x] = sum as u8;
                    }
                    5 => {
                        self.v[0xF] = if self.v[x] > self.v[y] { 1 } else { 0 };
                        self.v[x] = self.v[x].wrapping_sub(self.v[y]);
                    }
                    6 => {
                        self.v[0xF] = self.v[x] & 0x1;
                        self.v[x] >>= 1;
                    }
                    7 => {
                        self.v[0xF] = if self.v[y] > self.v[x] { 1 } else { 0 };
                        self.v[x] = self.v[y].wrapping_sub(self.v[x]);
                    }
                    0xE => {
                        self.v[0xF] = self.v[x] >> 7;
                        self.v[x] <<= 1;
                    }
                    _ => println!("Unknown opcode: {:#04x}", opcode),
                }
            }
            9 => {
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let y = ((opcode & 0x00F0) >> 4) as usize;
                if self.v[x] != self.v[y] {
                    self.pc += 2;
                }
            }
            0xA => self.i = opcode & 0x0FFF,
            0xB => self.pc = self.v[0] as u16 + (opcode & 0x0FFF),
            0xC => {
                let x = ((opcode & 0x0F00) >> 8) as usize;
                self.v[x] = random::<u8>() & (opcode & 0x00FF) as u8;
            }
            0xD => {
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let y = ((opcode & 0x00F0) >> 4) as usize;
                let n = (opcode & 0x000F) as usize;
                let vx = self.v[x] as usize % 64;
                let vy = self.v[y] as usize % 32;
                self.v[0xF] = 0;
                for row in 0..n {
                    let sprite_byte = self.mem[self.i as usize + row];
                    let y_pos = (vy + row) % 32;
                    for col in 0..8 {
                        if (sprite_byte >> (7 - col)) & 1 != 0 {
                            let x_pos = (vx + col) % 64;
                            let pixel_index = 63 - x_pos;
                            if (self.disp[y_pos] >> pixel_index) & 1 != 0 {
                                self.v[0xF] = 1;
                            }
                            self.disp[y_pos] ^= 1 << pixel_index;
                        }
                    }
                }
            }
            0xE => {
                let x = ((opcode & 0x0F00) >> 8) as usize;
                match opcode & 0x00FF {
                    0x9E => {
                        if self.keys[self.v[x] as usize] {
                            self.pc += 2;
                        }
                    }
                    0xA1 => {
                        if !self.keys[self.v[x] as usize] {
                            self.pc += 2;
                        }
                    }
                    _ => println!("Unknown opcode: {:#04x}", opcode),
                }
            }
            0xF => {
                let x = ((opcode & 0x0F00) >> 8) as usize;
                match opcode & 0x00FF {
                    0x07 => self.v[x] = self.delay,
                    0x0A => match self.keys.iter().position(|&k| k) {
                        Some(k) => self.v[x] = k as u8,
                        None => self.pc -= 2,
                    },
                    0x15 => self.delay = self.v[x],
                    0x18 => self.sound = self.v[x],
                    0x1E => self.i = self.i.wrapping_add(self.v[x] as u16),
                    0x29 => self.i = (self.v[x] as u16) * 5,
                    0x33 => {
                        self.mem[self.i as usize] = self.v[x] / 100;
                        self.mem[self.i as usize + 1] = (self.v[x] / 10) % 10;
                        self.mem[self.i as usize + 2] = self.v[x] % 10;
                    }
                    0x55 => {
                        for i in 0..=x {
                            self.mem[self.i as usize + i] = self.v[i];
                        }
                    }
                    0x65 => {
                        for i in 0..=x {
                            self.v[i] = self.mem[self.i as usize + i];
                        }
                    }
                    _ => println!("Unknown opcode: {:#04x}", opcode),
                }
            }
            _ => println!("Unknown opcode: {:#04x}", opcode),
        }
    }

    fn load_rom(&mut self) {
        // load ROM into memory using (cargo run -- "path/to/rom")
        let path = std::env::args().nth(1).expect("No ROM provided.");
        let rom = std::fs::read(path).expect("Error Loading ROM.");
        let end = 0x200 + rom.len();
        if end > 4096 {
            //if rom exceeds memory (4kb)
            panic!("ROM too large: {} bytes", rom.len());
        }
        self.mem[0x200..(0x200 + rom.len())].copy_from_slice(&rom);
    }
}

fn key_to_chip8(key: Key) -> Option<usize> {
    // keypad chip8
    match key {
        Key::Key1 => Some(0x1),
        Key::Key2 => Some(0x2),
        Key::Key3 => Some(0x3),
        Key::Key4 => Some(0xC),
        Key::Q => Some(0x4),
        Key::W => Some(0x5),
        Key::E => Some(0x6),
        Key::R => Some(0xD),
        Key::A => Some(0x7),
        Key::S => Some(0x8),
        Key::D => Some(0x9),
        Key::F => Some(0xE),
        Key::Z => Some(0xA),
        Key::X => Some(0x0),
        Key::C => Some(0xB),
        Key::V => Some(0xF),
        _ => None,
    }
}

fn main() {
    // main
    let mut chip8 = CHIP8::new();

    if std::env::args().nth(1).is_none() {
        println!("Usage: chip8 <rom> [cycles]");
        return;
    }

    chip8.load_rom();

    let cycles: usize = std::env::args() // cycles args
        .nth(2)
        .and_then(|s| s.parse().ok())
        .unwrap_or(10);
    let mut window = Window::new(
        // load new window
        "CHIP-8 Emulator",
        WIDTH,
        HEIGHT,
        WindowOptions {
            scale: minifb::Scale::X16, // scale bigger
            ..WindowOptions::default()
        },
    )
    .unwrap();
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT]; // buffer
    let mut last_timer = Instant::now(); // timer

    while window.is_open() {
        // on window open
        //sample keys once per frame
        chip8.keys = [false; 16];
        for key in window.get_keys() {
            if let Some(k) = key_to_chip8(key) {
                chip8.keys[k] = true;
            }
        }

        //run cycle
        for _ in 0..cycles {
            chip8.cycle();
        }

        // timer -1 at 60hz and beeps
        if last_timer.elapsed() > Duration::from_millis(16) {
            let now = Instant::now();
            let elapsed = now.duration_since(last_timer);
            let ticks = (elapsed.as_millis() / 16).min(5) as u8;
            if ticks > 0 {
                chip8.delay = chip8.delay.saturating_sub(ticks);
                if chip8.sound > 0 {
                    chip8.sound = chip8.sound.saturating_sub(ticks);
                    if chip8.sound == 0 {
                        println!("\x07");
                        std::io::stdout().flush().unwrap();
                    }
                }
                last_timer += Duration::from_millis(ticks as u64 * 16);
            }
        }

        //draw in window
        for y in 0..32 {
            for x in 0..64 {
                if chip8.disp[y] & (1 << (63 - x)) != 0 {
                    buffer[y * WIDTH + x] = 0xFFFFFFFF;
                } else {
                    buffer[y * WIDTH + x] = 0x00000000;
                }
            }
        }
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap(); // update
    }
}
