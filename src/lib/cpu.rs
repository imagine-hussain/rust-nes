use std::{rc::Rc, cell::RefCell};

use crate::{bus::Bus, RcCell};

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
    pub bus: RcCell<Bus>,
    pub clock: usize,
    pub a_register: u8,             // Accumulator
    pub x_register: u8,             // X Register
    pub y_register: u8,             // Y Register
    pub stack_pointer: u8,          // Stack Pointer
    pub program_counter: u16,       // Program Counter
    pub status_register: u8,        // Status Register
    pub fetched_data: u8,           // Fetched data - temp storage
}

impl Cpu {
    pub fn new() -> RcCell<Self> {
        let bus = Rc::new(RefCell::new(Bus::new()));

        let new_cpu = Rc::new(RefCell::new(Self {
            bus,
            clock: 0,
            a_register: 0,
            x_register: 0,
            y_register: 0,
            stack_pointer: 0,
            program_counter: 0,
            status_register: 0,
            fetched_data: 0,
        }));

        new_cpu.borrow_mut().bus.borrow_mut().connect_cpu(new_cpu.clone());

        new_cpu
    }

    pub fn get_bus(&self) -> RcCell<Bus> {
        self.bus.clone()
    }

    pub fn read(&self, address: u16) -> u8 {
        self.bus.borrow().read(address)
    }

    pub fn write(&mut self, address: u16, data: u8) {
        self.bus.borrow_mut().write(address, data)
    }

    pub fn clock(&mut self) {
        self.clock += 1
    }

    pub fn clock_n(&mut self, cycles: usize) {
        self.clock += cycles
    }

    pub fn reset() { todo!() }
    pub fn interrupt_request() { todo!() }
    pub fn non_maskabl_interrupt_request() { todo!() }

    // fn fetch_data() -> u8 { todo!() }

}

pub enum CpuFlag {
    Carry = 1 << 0,
    Zero = 1 << 1,          // Set when result of operation is 0
    Interrupt = 1 << 2,     // Disable interrupts; TODO: what is order?
    Decimal = 1 << 3,       // If in Decimal mode; TODO: unused
    Break = 1 << 4,         // Set when a break instruction is executed
    Unused = 1 << 5,        // Unused
    Overflow = 1 << 6,      // Set when an overflow occurs. Only when using signed values
    Negative = 1 << 7,      // Set when the result of an operation is negative
}

pub fn set_flag(flag: CpuFlag, status_register: u8) -> u8 {
    status_register | flag as u8
}

// Addressing Modes:
// IMP: Implied
// IMM: Immediate
// ZP0: Zero Page
// ZPX: Zero Page, X
// ZPY: Zero Page, Y
// REL: Relative
// ABS: Absolute
// ABX: Absolute, X
// ABY: Absolute, Y
// IND: Indirect
// IZX: Indirect, X
// IZY: Indirect, Y

