#[derive(Default, Debug, Clone)]
pub(super) struct Registers {
    v: u128,
    r: u64
}

impl Registers {
    // Get u8 representing register Vx
    fn vx(&self, x: u8) -> u8 {
        ((self.v >> (x * 8)) & 0xFF) as u8
    }

    // Write u8 w to register Vx
    fn wvx(&mut self, x: u8, w: u8) -> u8 {
        self.v &= u128::MAX - 0x00 << (x * 8);
        self.v |= (w as u128) << (x * 8) as u128;
        self.vx(x)
    }

    // Add u8 w to register Vx
    fn avx(&mut self, x: u8, w: u8) -> u8 {
        self.wvx(x, self.vx(x) + w)
    }

    // Get the PC
    fn pc(&self) -> u16 {
        (self.r & 0xFFFF) as u16
    }

    // Write to PC

    // Increment the PC

    // Get the Sprite Index
    fn i(&self) -> u16 {
        (self.r & (0xFFFF << 16)) as u16
    }
    
    // Write to the Sprite Index

    // Get the Stack Pointer
    fn sp(&self) -> u8 {
        (self.r & (0xFF << 24)) as u8
    }

    // Get the delay timer
    fn delay(&self) -> u8 {
        (self.r & (0xFF << 32)) as u8
    }

    //  Get the sound timer
    fn sound(&self) -> u8 {
        (self.r & (0xFF << 40)) as u8
    }
}