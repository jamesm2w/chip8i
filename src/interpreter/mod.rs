use chip8_base::{Interpreter, Display, Pixel};

pub mod register;

// The Interpreter for Chip8
pub struct Chip8I {
    memory: [u8; 4096], // 4KB of addr memory
    registers: [u8; 16], // 16 8-bit registers V0-VF (VF being flags, etc)
    i: u16, // Special addr register
    pc: u16,
    sp: u8,
    delay: u8, // Delay timer
    sound: u8, // Sound timer
    display: Display // Display store
}

impl Chip8I {
    pub fn new() -> Self {
        Self { memory: [0; 0x1000], registers: [0; 16], i: 0, pc: 0x200, sp: 0, delay: 0, sound: 0, display: [[Pixel::Black; 64]; 32] }
    }

    pub fn fetch(&mut self) -> u8 {
        let instr = self.memory[self.pc as usize];
        // println!("instr {:?}", instr);
        self.pc += 1;
        if self.pc >= 4096 {
            self.pc = 0;
        }

        instr
    }

    pub fn execute(&mut self, instr_a: u8, instr_b: u8) {
        let ns = ((instr_a & 0xF0) >> 4, instr_a & 0x0F, (instr_b & 0xF0) >> 4, instr_b & 0x0F);
        // println!("{:?} {:?} {:?}", instr_a, instr_b, ns);
        match ns {
            (0x0, 0x0, 0x0, 0x0) => println!("NULL OP"),
            (0x0, 0x0, 0xE, 0x0) => {
                // println!("CLS");
                for x in 0..32 {
                    for y in 0..64 {
                        self.display[x][y] = Pixel::Black;
                    }
                }
            },
            (0x1, n2, n1, n0) => {
                // println!("JMP {:?}{:?}{:?}", n2, n1, n0);
                self.pc = (n2 as u16) << 8;
                self.pc |= (n1 as u16) << 4;
                self.pc |= n0 as u16;
            },
            (0x6, nk, b1, b0) => {
                // println!("SET {:?}{:?}{:?}", nk, b1, b0);
                self.registers[nk as usize] = b1 << 4;
                self.registers[nk as usize] |= b0;
            },
            (0x7, nk, b1, b0) => {
                // println!("ADD {:?}{:?}{:?}", nk, b1, b0);
                self.registers[nk as usize] = self.registers[nk as usize].wrapping_add((b1 << 4) | b0);
            },
            (0xA, n2, n1, n0) => {
                // println!("INDEX {:#05x}", ((n2 as u16) << 8) | ((n1 as u16) << 4) | (n0 as u16));
                self.i = ((n2 as u16) << 8) | ((n1 as u16) << 4) | (n0 as u16);
            },
            (0xD, nx, ny, nn) => {
                // println!("DRAW {:?} {:?} {:?}", nx, ny, nn);
                let x = self.registers[nx as usize] % 64;
                let y = self.registers[ny as usize] % 32;
                // println!("x, y: {:?} {:?}", x, y);

                self.registers[0xF as usize] = 0;

                for i in 0..nn { 
                    let sprite = self.memory[self.i as usize + i as usize];
                
                    // if i + y >= 32 { return; }

                    for j in 0..8 {
                        // if j + x >= 64 { return; }
                        // println!("coord {:?},{:?}", i , j );
                        let pixel = ( sprite >> 7-j ) & 0x1;
                        let before = self.display[(i + y) as usize % 32][(j + x) as usize % 64];

                        self.display[(i + y) as usize % 32][(j + x) as usize % 64] ^= Pixel::try_from(pixel).unwrap(); 
                        
                        let after = self.display[(i + y) as usize % 32][(j + x) as usize % 64];
                        if before == Pixel::White && after == Pixel::Black {
                            self.registers[0xF as usize] = 1;
                        }
                    }
                }
            }
            _ => ()
        }
    }

    pub fn read_rom(&mut self, rom: Vec<u8>) {
        let mut index = 0x0200; // ROM loaded at 0x200
        for i in 0..rom.len() {
            self.memory[index] = rom[i];
            index += 1;

            if index >= 4096 {
                panic!("ROM too big");
            }
        }
    }

    pub fn print_memdump(&self) {
        let mut i = 0;
        for byte in self.memory.chunks(8) {
            println!("{:#05x} [{:#04x} {:#04x} {:#04x} {:#04x} {:#04x} {:#04x} {:#04x} {:#04x}] {:#05x}", i, byte[0], byte[1], byte[2], byte[3], byte[4], byte[5], byte[6], byte[7], i+7);
            i += 8;
        }
        // println!("{:?}", self.memory.chunks(8).collect::<Vec<_>>());
    }
}

impl Interpreter for Chip8I {
    fn step(&mut self, _keys: &chip8_base::Keys) -> Option<Display> {
        let instra = self.fetch();
        let instrb = self.fetch();
        self.execute(instra, instrb);
        Some(self.display)
    }

    fn speed(&self) -> std::time::Duration {
        std::time::Duration::from_millis(50)
    }

    fn buzzer_active(&self) -> bool {
        false
    }
}