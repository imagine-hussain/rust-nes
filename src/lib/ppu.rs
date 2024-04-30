

use crate::{Cartridge, RcCell, Reset};

/// # Emulation Structure for the Picture Processing Unit (PPU)
///
/// <https://www.nesdev.org/wiki/PPU>
///
/// ## Memory Mapping
/// The PPU has a an address space of 18kB. That is 0x0000 - 0x3FFF.
/// This memory is accessible internally by the PPU or, externally, by the CPU,
/// through the eight memory mapped registers at 0x2000 - 0x2007. in the CPU's
/// address space. (Not that these are mirrored every 8 bytes from 0x2000-0x3FFF)
///
/// ## Pallete Memory Map
/// | Address         | Description                     |
/// |-----------------|---------------------------------|
/// | 0x3F00	      | Universal background color      |
/// | 0x3F01 - 0x3F03 | Background palette 0            |
/// | 0x3F05 - 0x3F07 | Background palette 1            |
/// | 0x3F09 - 0x3F0B | Background palette 2            |
/// | 0x3F0D - 0x3F0F | Background palette 3            |
/// | 0x3F11 - 0x3F13 | Sprite palette 0                |
/// | 0x3F15 - 0x3F17 | Sprite palette 1                |
/// | 0x3F19 - 0x3F1B | Sprite palette 2                |
/// | 0x3F1D - 0x3F1F | Sprite palette 3                |
///
pub struct Ppu {
    // Physical parts of the NES
    // Has got 10Kb of memory. Split up into into the following address spaces.
    _name_table: [u8; 2 * 1024], // 1kb NameTable * 2
    memory: [u8; 10 * 1024],     // TODO: Stand-in for better management
    _palette: [u8; 32],          // 32 bytes of Palette (8 blocks of 4 bytes)
    pub cartridge: Option<RcCell<Cartridge>>,
    scanline: usize,
    cycle: usize,
    //
    /// A tile is a 8x8 bitmap, where each pixel takes 2 bits, giving it access
    /// to 4 colors.
    /// The two bits are stored in a LSB and MSB bit-plane.
    /// It takes 16 bytes to store a tile.
    /// To calculate the *actual* colour of a pixel, we need to look up the
    /// pallete information.
    pub pattern: [u8; 8 * 1024],
    // name_table: [u8; 2 * 1024],
    // palette: [u8; 32],
}

impl Ppu {
    pub const SCREEN_WIDTH: usize = 340;
    pub const SCREEN_HEIGHT: usize = 240;

    pub fn new() -> Self {
        Self {
            cartridge: None,
            _name_table: [0; 2 * 1024],
            _palette: [0; 32],
            memory: [0; 10 * 1024],
            scanline: 0,
            pattern: [0; 8 * 1024],
            cycle: 0,
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

    pub fn insert_cartidge(&mut self, cartridge: Option<RcCell<Cartridge>>) {
        self.cartridge = cartridge;
    }

    pub fn tick(&mut self) {
        // TODO: Render to some screen

        // Advance to next pixel and wrap column and scanline around
        self.cycle += 1;
        if self.cycle > Self::SCREEN_WIDTH {
            self.cycle = 0;
            self.scanline += 1;
            if self.scanline > Self::SCREEN_HEIGHT {
                self.scanline = 0;
            }
        }
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
