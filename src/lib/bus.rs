use std::{cell::Ref, ops::RangeInclusive};

use crate::{Cartridge, Clock, Cpu, Ppu, RcCell, Reset, WeakCell};

/// # Bus For NES
///
/// Connects all the components of the NES together, allowing them to communicate
/// with each other.
///
/// ## Memory Layout - CPU
///
///
///
/// | Range           | Size | Description                                   |
/// |-----------------|------|-----------------------------------------------|
/// | 0x0000 - 0x07FF | 2KB  | Internal Cpu Ram                              |
/// | 0x0800 - 0x0FFF | 2KB  | Mirror of Cpu Ram                             |
/// | 0x1000 - 0x17FF | 2KB  | Mirror of Cpu Ram                             |
/// | 0x1800 - 0x1FFF | 2KB  | Mirror of Cpu Ram                             |
/// | 0x2000 - 0x2007 | 8B   | PPU Registers                                 |
/// | 0x2008 - 0x3FFF | 8KB  | Mirrors of PPU Registers                      |
/// | 0x4000 - 0x4017 | 24B  | APU and I/O Registers                         |
/// | 0x4018 - 0x401F | 8B   | For Cpu Test Mode by APU / IO                 |
/// | 0x4020 - 0xFFFF | 48KB | Cartridge Space (PRG ROM, PRG RAM, Mapper Reg)|
/// | 0xFFFA - 0xFFFB | 2B   | NMI Vector                                    |
/// | 0xFFFC - 0xFFFD | 2B   | Reset Vector                                  |
/// | 0xFFFE - 0xFFFF | 2B   | IRQ / BRK Vector                              |
///
pub struct Bus {
    pub cpu: WeakCell<Cpu>,
    pub ppu: WeakCell<Ppu>,
    /// See Memory Layout - CPU
    pub ram: [u8; 64 * 1024],
    pub cartridge: Option<RcCell<Cartridge>>,
    pub clock: Clock,
}

impl Bus {
    const CPU_RAM_START: u16 = 0x0000;
    const CPU_RAM_END: u16 = 0x1FFF;
    pub const CPU_RAM_RANGE: RangeInclusive<u16> = Self::CPU_RAM_START..=Self::CPU_RAM_END;
    pub const CPU_RAM_MIRROR_MASK: u16 = 0x07FF;

    const PPU_START: u16 = 0x2000;
    const PPU_END: u16 = 0x3FFF;
    pub const PPU_RANGE: RangeInclusive<u16> = Self::PPU_START..=Self::PPU_END;
    const PPU_MEMORY_MASK: u16 = 0b111;

    pub fn new(cpu: WeakCell<Cpu>, ppu: WeakCell<Ppu>) -> Self {
        Self {
            cpu,
            ram: [0; 64 * 1024],
            ppu,
            cartridge: None,
            clock: Clock::default(),
        }
    }

    pub fn get_cpu(&self) -> Option<RcCell<Cpu>> {
        self.cpu.upgrade()
    }

    pub fn unwrap_cpu(&self) -> RcCell<Cpu> {
        self.cpu.upgrade().unwrap()
    }

    pub fn get_ppu(&self) -> Option<RcCell<Ppu>> {
        self.ppu.upgrade()
    }

    pub fn unwrap_ppu(&self) -> RcCell<Ppu> {
        self.ppu.upgrade().unwrap()
    }

    pub fn connect_cpu(&mut self, cpu: WeakCell<Cpu>) {
        self.cpu = cpu;
    }

    pub fn connect_ppu(&mut self, ppu: WeakCell<Ppu>) {
        self.ppu = ppu;
    }

    /// Connects PPU to CPU BUS
    pub fn write_cpu(&mut self, address: u16, data: u8) {
        if Self::CPU_RAM_RANGE.contains(&address) {
            self.ram[(address as usize) & (Self::CPU_RAM_MIRROR_MASK as usize)] = data;
        } else if Self::PPU_RANGE.contains(&address) {
            self.unwrap_ppu()
                .borrow_mut()
                .write_cpu(address & Self::PPU_MEMORY_MASK, data);
        } else {
            // panic!("Unimplemented write to address: {:04X}", address);
            // 0
        }
    }

    /// Connects PPU to CPU BUS
    pub fn read_cpu(&self, address: u16) -> u8 {
        if Self::CPU_RAM_RANGE.contains(&address) {
            self.ram[(address & Self::CPU_RAM_MIRROR_MASK) as usize]
        } else if Self::PPU_RANGE.contains(&address) {
            self.unwrap_ppu()
                .borrow_mut()
                .read_cpu(address & Self::PPU_MEMORY_MASK)
        } else {
            // panic!("Unimplemented read of address: {:04X}", address);
            0
        }
    }

    pub fn tick(&mut self) {
        self.clock.tick();
    }

    pub fn insert_cartridge(&mut self, cartridge: Option<RcCell<Cartridge>>) {
        self.cartridge = cartridge;
    }

    pub fn cartridge_ref(&self) -> Option<Ref<Cartridge>> {
        self.cartridge.as_ref().map(|cartridge| cartridge.borrow())
    }
}

impl Default for Bus {
    fn default() -> Self {
        Self {
            cpu: WeakCell::new(),
            ppu: WeakCell::new(),
            ram: [0; 64 * 1024],
            clock: Clock::default(),
            cartridge: None,
        }
    }
}

impl Reset for Bus {
    fn reset(&mut self) {
        self.clock.reset();
    }
}
