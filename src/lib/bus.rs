use std::{rc::{Rc, Weak}, cell::RefCell};

use crate::cpu::Cpu;

pub struct Bus {
    pub cpu: Weak<RefCell<Cpu>>,
    pub ram: [u8; 64 * 1024],
}

impl Bus {

    pub fn new() -> Self {
        Self {
            cpu: Weak::new(),
            ram: [0; 64 * 1024],
        }
    }

    pub fn get_cpu(&self) -> Option<Rc<RefCell<Cpu>>> {
        self.cpu.upgrade()
    }

    pub fn unwrap_cpu(&self) -> Rc<RefCell<Cpu>> {
        self.cpu.upgrade().unwrap()
    }

    pub fn connect_cpu(&mut self, cpu: Rc<RefCell<Cpu>>) {
        self.cpu = Rc::downgrade(&cpu);
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


