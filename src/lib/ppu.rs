use crate::{Cartridge, RcCell, Reset};

/// # Emulation Structure for the Picture Processing Unit (PPU)
///
/// <https://www.nesdev.org/wiki/PPU>
pub struct Ppu {
    // Physical parts of the NES
    // Has got 10Kb of memory. Split up into into the following address spaces.
    name_table: [u8; 2 * 1024], // 1kb NameTable * 2
    memory: [u8; 10 * 1024],     // TODO: Stand-in for better management
    palette: [u8; 32],          // 32 bytes of Palette (8 blocks of 4 bytes)
    pub cartridge: Option<RcCell<Cartridge>>,
    // pattern: [u8; 8 * 1024],
    // name_table: [u8; 2 * 1024],
    // palette: [u8; 32],
}

impl Ppu {
    pub fn new() -> Self {
        Self {
            cartridge: None,
            name_table: todo!(),
            palette: todo!(),
            memory: todo!(),
        }
    }

    pub fn write_cpu(&mut self, address: u16, data: u8) {
        // WRONG!
        // TODO: Fix with the 8 cases
        self.memory[address as usize] = data;
    }

    pub fn read_cpu(&self, address: u16) -> u8 {
        // TODO: Fix with the 8 cases
        self.memory[address as usize]
    }

    pub fn insert_cartidge(&mut self, cartridge: RcCell<Cartridge>) {
        self.cartridge = Some(cartridge);
    }

    pub fn tick(&mut self) {

    }
}

impl Default for Ppu {
    fn default() -> Self {
        Ppu::new()
    }
}

impl Reset for Ppu {
    fn reset(&mut self) {
        // TODO: Implement a reset
        // self.memory = [0; 10 * 1024];
    }
}

