use crate::Mapper;

/// # Mapper000 (NROM)
/// The generic designation NROM refers to the Nintendo cartridge boards NES-NROM-128,
/// NES-NROM-256, their HVC counterparts, and clone boards.
/// Mapper000 as per the standardised Nes Mapper guidelines on nesdev.org
///
/// <https://www.nesdev.org/wiki/NROM>
///
/// ## Specification
/// Supports 16K or 32K PRG-ROM and 8K CHR-ROM.
/// Uses a 32K address space for PRG-ROM regardless. If the ROM is only 16K
/// then the last 16K is a mirror of the first 16K.
/// (CPU)First 16K: 0x8000 - 0xBFFF
/// (CPU)Last  16K: 0xC000 - 0xFFFF
///
pub struct Mapper000 {
    pub prg_banks_count: u8,
    pub chr_banks_count: u8,
    pub mirrored: bool,
}

impl Mapper for Mapper000 {
    fn new(header: &crate::cartridge::Header) -> Self {
        Self {
            prg_banks_count: header.prg_rom_size,
            chr_banks_count: header.prg_chr_size,
            mirrored: header.prg_rom_size == 16,
        }
    }

    fn map_cpu_read(&mut self, addr: u16, new_addr: &mut u16) -> bool {
        if !(0x8000..=0xFFFF).contains(&addr) {
            return false;
        }

        *new_addr = match self.mirrored {
            true => addr & 0x7FFF,
            false => addr & 0x3FFF,
        };

        true
    }

    fn map_cpu_write(&mut self, addr: u16, new_addr: &mut u16) -> bool {
        if !(0x8000..=0xFFFF).contains(&addr) {
            return false;
        }

        *new_addr = match self.mirrored {
            true => addr & 0x7FFF,
            false => addr & 0x3FFF,
        };

        true
    }

    fn map_ppu_read(&mut self, addr: u16, new_addr: &mut u16) -> bool {
        match (0x1FFF..=0x8000).contains(&addr) {
            true => {
                *new_addr = addr;
                true
            }
            false => false,
        }
    }

    fn map_ppu_write(&mut self, addr: u16, new_addr: &mut u16) -> bool {
        match (0x1FFF..=0x8000).contains(&addr) {
            true => {
                *new_addr = addr;
                true
            }
            false => false,
        }
    }
}
