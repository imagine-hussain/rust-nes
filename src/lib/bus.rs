use std::ops::RangeInclusive;

use crate::{Cartridge, Clock, Cpu, Ppu, RcCell, Reset, WeakCell};

pub struct Bus {
    pub cpu: WeakCell<Cpu>,
    pub ppu: WeakCell<Ppu>,
    pub ram: [u8; 64 * 1024],
    pub cartridge: Option<RcCell<Cartridge>>,
    pub clock: Clock,
}

impl Bus {
    const RAM_START: u16 = 0x0000;
    const RAM_END: u16 = 0x1FFF;
    pub const RAM_RANGE: RangeInclusive<u16> = Self::RAM_START..=Self::RAM_END;
    pub const RAM_MIRROR_MASK: u16 = 0x07FF;

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

    pub fn write_cpu(&mut self, address: u16, data: u8) {
        if Self::RAM_RANGE.contains(&address) {
            self.ram[(address as usize) & (Self::RAM_MIRROR_MASK as usize)] = data;
        } else if Self::PPU_RANGE.contains(&address) {
            self.unwrap_ppu()
                .borrow_mut()
                .write_cpu(address & Self::PPU_MEMORY_MASK, data);
        } else {
            panic!("Unimplemented write to address: {:04X}", address);
        }
    }

    pub fn read_cpu(&self, address: u16) -> u8 {
        if Self::RAM_RANGE.contains(&address) {
            self.ram[(address as usize) & (Self::RAM_MIRROR_MASK as usize)]
        } else if Self::PPU_RANGE.contains(&address) {
            self.unwrap_ppu()
                .borrow_mut()
                .read_cpu(address & Self::PPU_MEMORY_MASK)
        } else {
            0
        }
    }

    pub fn tick(&mut self) {
        self.clock.tick();
    }

    pub fn insert_cartridge(&mut self, cartridge: Option<RcCell<Cartridge>>) {
        self.cartridge = cartridge;
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
