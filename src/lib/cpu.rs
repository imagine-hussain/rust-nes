use std::{rc::Rc, cell::RefCell};

use crate::bus::Bus;

/// Emulator for the `6502` CPU.
///
/// DataSheet:
/// http://archive.6502.org/datasheets/rockwell_r650x_r651x.pdf
///
/// Datasheet of Rockewell 65c00 (not what's being emulated but, is backwards compatiable)
///
///
/// Notes:
///
/// Can output addresses
/// Can read/write data
/// Need a signal for it its reading / writing -> not needed in impl
/// need a clock and a curr_pointer
///
///
/// addresses are 16 bit
///
///
///  -Ram - 64k

/// 56 instructions
/// Care about the size (1-3 bytes) and how many cycles it takes to execute (duration)
/// First byte lets us know what instruction size we are dealing with
///
///



/// Registers:
/// A: Accumulator
/// X: Register
/// Y: Register
/// SP: Stack pointer
/// PC: Program Counter
/// Status: Status register
pub struct Cpu {
    bus: Rc<RefCell<Bus>>
}

impl Cpu {
    pub fn new() -> Self {
        let bus = Rc::new(RefCell::new(Bus::new()));

        let new_cpu = Self {
            bus
        };

        new_cpu.bus.borrow_mut().connect_cpu(&new_cpu);


        new_cpu
    }


    pub fn read(&self, address: u16) -> u8 {
        self.bus.borrow().read(address)
    }

    pub fn write(&mut self, address: u16, data: u8) {
        self.bus.borrow_mut().write(address, data)
    }

}

impl Default for Cpu {
    fn default() -> Self {
        Self::new()
    }
}

