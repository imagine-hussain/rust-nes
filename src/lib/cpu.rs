use crate::{bus::Bus, opcodes::opcode_types::OpCodeType, AddressingMode, RcCell};
use std::{cell::RefCell, rc::Rc};

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
    pub a_register: u8,       // Accumulator
    pub x_register: u8,       // X Register
    pub y_register: u8,       // Y Register
    pub stack_pointer: u8,    // Stack Pointer
    pub program_counter: u16, // Program Counter
    pub status_register: u8,  // Status Register
    ///
    /// Not "real" parts of the hardware but for emulation
    pub fetched_data: u8, // Fetched data - temp storage
    pub absolute_addr: u16,   // Absolute address being read off
    pub relative_addr: i8,    // Address relative to abs address
    pub addressing_mode: AddressingMode, // Addressing mode
}

impl Cpu {

    pub const STACK_BASE: u16 = 0x0100;

    pub fn new() -> RcCell<Self> {
        let tmp_bus = Rc::new(RefCell::new(Bus::default()));

        let new_cpu = Rc::new(RefCell::new(Self {
            bus: tmp_bus,
            clock: 0,
            a_register: 0,
            x_register: 0,
            y_register: 0,
            stack_pointer: 0,
            program_counter: 0,
            status_register: 0,
            fetched_data: 0,
            absolute_addr: 0,
            relative_addr: 0,
            addressing_mode: AddressingMode::IMP,
        }));

        new_cpu
            .borrow_mut()
            .bus
            .borrow_mut()
            .connect_cpu(Rc::downgrade(&new_cpu));

        new_cpu
    }

    /// Fetch based on the current addressing mode. Stored in `self.fetched_data`
    /// Also returns the fetched_data
    pub fn fetch(&mut self) -> u8 {
        // todo: special case for imp
        self.addressing_mode.fetch()(self);
        self.fetched_data
    }

    pub fn execute_clock_cycle(&mut self) {
        // Fetch
        // let raw_opcode = self.read(self.program_counter);
        // self.program_counter += 1;
        //
        // // Set unused flag
        // self.status_register = set_flag(CpuFlag::Unused, self.status_register);
        // let opcode: OpCodeType = raw_opcode.into();
        //
        // // Execute what is required for the operation including address mode operations
        // let additional_cycle_addrmode = opcode.addr_mode(self);
        // let additional_cycle_operation = opcode.execute(self);
        // if additional_cycle_addrmode && additional_cycle_operation {
        //     self.clock += 1;
        // }

        // Decode
        // Execute
        // Increment PC
    }

    #[inline(always)]
    pub fn get_bus(&self) -> RcCell<Bus> {
        self.bus.clone()
    }

    #[inline(always)]
    pub fn read(&self, address: u16) -> u8 {
        self.bus.borrow().read(address)
    }

    #[inline(always)]
    pub fn write(&mut self, address: u16, data: u8) {
        self.bus.borrow_mut().write(address, data)
    }

    #[inline(always)]
    pub fn push_stack(&mut self, data: u8) {
        self.write(Cpu::STACK_BASE + self.stack_pointer as u16, data);
        self.stack_pointer -= 1;
    }

    #[inline(always)]
    pub fn pop_stack(&mut self) -> u8 {
        self.stack_pointer += 1;
        self.read(Cpu::STACK_BASE + self.stack_pointer as u16)
    }

    #[inline(always)]
    pub fn clock(&mut self) {
        self.clock += 1
    }

    #[inline(always)]
    pub fn clock_n(&mut self, cycles: usize) {
        self.clock += cycles
    }

    pub fn reset() {
        todo!()
    }
    pub fn interrupt_request() {
        todo!()
    }
    pub fn non_maskable_interrupt_request() {
        todo!()
    }

    #[inline]
    pub fn set_flag(&mut self, flag: &CpuFlag) {
        self.status_register = set_flag(&self.status_register, flag)
    }

    #[inline]
    pub fn clear_flag(&mut self, flag: &CpuFlag) {
        self.status_register = clear_flag(&self.status_register, flag)
    }

    #[inline]
    pub fn get_flag(&self, flag: &CpuFlag) -> bool {
        (self.status_register & *flag as u8) != 0
    }

    pub fn set_or_clear_flag(&mut self, flag: &CpuFlag, set: bool) {
        if set {
            self.set_flag(flag)
        } else {
            self.clear_flag(flag)
        }
    }

    // fn fetch_data() -> u8 { todo!() }
}

#[derive(Debug, Clone, Copy)]
pub enum CpuFlag {
    Carry = 1 << 0,
    Zero = 1 << 1,      // Set when result of operation is 0
    Interrupt = 1 << 2, // Disable interrupts; TODO: what is order?
    Decimal = 1 << 3,   // If in Decimal mode; TODO: unused
    Break = 1 << 4,     // Set when a break instruction is executed
    Unused = 1 << 5,    // Unused
    Overflow = 1 << 6,  // Set when an overflow occurs. Only when using signed values
    Negative = 1 << 7,  // Set when the result of an operation is negative
}

pub fn set_flag(status_register: &u8, flag: &CpuFlag) -> u8 {
    *status_register | *flag as u8
}

pub fn clear_flag(status_register: &u8, flag: &CpuFlag) -> u8 {
    *status_register & (!(*flag as u8))
}
