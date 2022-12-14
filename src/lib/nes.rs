use std::{rc::Rc, cell::RefCell};

use crate::{Bus, Clock, Cpu, Ppu, RcCell, Cartridge, Reset};

pub struct Nes {
    pub cpu: RcCell<Cpu>,
    pub bus: RcCell<Bus>,
    pub ppu: RcCell<Ppu>,
    pub clock: Clock,
}

impl Nes {
    fn new(cpu: RcCell<Cpu>, bus: RcCell<Bus>, ppu: RcCell<Ppu>, clock: Clock) -> Self {
        Self {
            cpu,
            bus,
            ppu,
            clock,
        }
    }

    pub fn tick(&mut self) {
        self.bus.borrow_mut().tick();
        self.ppu.borrow_mut().tick();

        // Cpu is 3 times slower than PPU
        if self.clock.total_ticks() % 3 == 0 {
            self.cpu.borrow_mut().tick();
        }

        self.clock.tick();
    }

    pub fn insert_cartidge(&mut self, cartridge: RcCell<Cartridge>) {
        self.bus.borrow_mut().insert_cartridge(cartridge.clone());
        self.ppu.borrow_mut().insert_cartidge(cartridge);
    }

}

impl Default for Nes {
    fn default() -> Self {
        let cpu = Cpu::new();
        let bus = cpu.borrow().get_bus();
        let clock = Clock::default();
        let ppu = Rc::new(RefCell::new(Ppu::default()));

        Self::new(cpu, bus, ppu, clock)
    }
}

impl Reset for Nes {
    fn reset(&mut self) {
        self.cpu.borrow_mut().reset();
        self.bus.borrow_mut().reset();
        self.ppu.borrow_mut().reset();
        self.clock.reset();
    }
}

