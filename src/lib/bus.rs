use std::rc::{Rc, Weak};

use crate::cpu::Cpu;

pub struct Bus {
    pub cpu: Weak<Cpu>,
    pub ram: [u8; 64 * 1024],
}

impl Bus {

    pub fn new() -> Self {
        Self {
            cpu: Weak::new(),
            ram: [0; 64 * 1024],
        }
    }

    pub fn connect_cpu(&mut self, cpu: &Cpu) {
        self.cpu = Rc::downgrade(&Rc::new(Cpu::new()));
    }

    pub fn write(&mut self, address: u16, data: u8) {
        if (0x0000..=0xFFFF).contains(&address) {
            self.ram[address as usize] = data;
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        if (0x0000..=0xFFFF).contains(&address) {
            self.ram[address as usize]
        } else {
            0
        }
    }

}

impl Default for Bus {
    fn default() -> Self {
        Self::new()
    }
}


