use crate::{
    clock::Clock,
    cpu::{
        flags::{clear_flag, set_flag, CpuFlag},
        AddressingMode,
    },
    opcodes::OpCode,
    Bus, RcCell, Reset,
};
use std::{cell::RefCell, fmt::format, rc::Rc};

/// Emulator for the `6502` CPU.
///
/// DataSheet:
/// <http://archive.6502.org/datasheets/rockwell_r650x_r651x.pdf>
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

/// Registers:
/// A: Accumulator
/// X: Register
/// Y: Register
/// SP: Stack pointer
/// PC: Program Counter
/// Status: Status register
pub struct Cpu {
    pub bus: RcCell<Bus>,
    pub clock: Clock,
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
    pub additional_cycle_addrmode: u8, // Additional cycles for addressing mode
    pub additional_cycle_operation: u8, // Additional cycles for operation
}

impl Cpu {
    pub const STACK_BASE: u16 = 0x0100;
    pub const RESET_VECTOR: u16 = 0xFFFC;
    pub const STACK_POINTER_RESET: u8 = 0xFD;
    pub const IRQ_VECTOR: u16 = 0xFFFE;
    pub const NMI_VECTOR: u16 = 0xFFFA;

    pub fn new() -> RcCell<Self> {
        let tmp_bus = Rc::new(RefCell::new(Bus::default()));

        let new_cpu = Rc::new(RefCell::new(Self {
            bus: tmp_bus,
            clock: Clock::default(),
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
            additional_cycle_addrmode: 0,
            additional_cycle_operation: 0,
        }));

        new_cpu
            .borrow_mut()
            .bus
            .borrow_mut()
            .connect_cpu(Rc::downgrade(&new_cpu));

        new_cpu
    }

    /// Emulate a single clock cycle. This does not translate to one
    /// instruction.
    /// The CPU will execute an instruction over multiple clock cycles.
    /// However, the emulation occurs on a per-instruction basis, rather
    /// than a per-clock cycle basis.
    /// As such, if an instructions takes `n` clock cycles, `1` call of this
    /// will execute the instruction and there will be `n - 1` calls to this
    /// that do nothing (except increment the clock).
    pub fn tick(&mut self) {
        if !self.clock.is_ready() {
            self.clock.tick();
            // return;
        }
        self.execute_clock_cycle();
    }

    pub fn execute_clock_cycle(&mut self) {
        // Not ready yet.

        // Fetch next instruction
        let opcode: OpCode = self.read(self.program_counter).into();
        // println!("Instruction:\t {opcode:?}");
        self.program_counter += 1;

        // Always unused
        self.set_flag(&CpuFlag::Unused);

        // Initial (minimum) amount of cycles - May be extra later
        self.clock.set_cycles(opcode.cycles as u64);

        // Addressing Mode Lookup for `absolute_addr` or `relative_addr`
        self.addressing_mode = opcode.addressing_mode;
        // FIXME: Not sure if this is going to break things by doing a pre-emptive
        // fetch of the data, since the operation_fn also fetches.
        // PC may move out of sync.
        // Might want to do this in the actual addressing mode functions.
        self.additional_cycle_addrmode = self.addressing_mode.fetch()(self);

        // Do the operation
        self.additional_cycle_operation = opcode.code_type.executable()(self);

        // Calculate total cycles;
        // Refactor: Don't use members but actually return / pass the data around
        let additional_cycles = self.additional_cycle_operation & self.additional_cycle_addrmode;
        self.clock.add_cycles(additional_cycles as u64);

        // println!("reached end");
    }

    /// Fetch based on the current addressing mode. Stored in `self.fetched_data`
    /// Also returns the fetched_data
    pub fn fetch(&mut self) -> u8 {
        // todo: special case for imp
        self.addressing_mode.fetch()(self);
        self.fetched_data
    }

    #[inline(always)]
    pub fn get_bus(&self) -> RcCell<Bus> {
        self.bus.clone()
    }

    #[inline(always)]
    pub fn read(&self, address: u16) -> u8 {
        self.bus.borrow().read_cpu(address)
    }

    #[inline(always)]
    pub fn write(&mut self, address: u16, data: u8) {
        self.bus.borrow_mut().write_cpu(address, data)
    }

    #[inline(always)]
    pub fn push_stack(&mut self, data: u8) {
        self.write(Cpu::STACK_BASE + self.stack_pointer as u16, data);
        self.stack_pointer = self.stack_pointer.wrapping_sub(1);
    }

    #[inline(always)]
    pub fn pop_stack(&mut self) -> u8 {
        self.stack_pointer += 1;
        self.read(Cpu::STACK_BASE + self.stack_pointer as u16)
    }

    /// Trigger an interrupt if the disable interrupt flag is not set.
    /// They happen at any time but, we want to only execute these between
    /// instructions
    /// TODO: In future, an event queue can be very useful to handle this
    /// trickiness
    pub fn interrupt_request(&mut self) {
        if self.get_flag(&CpuFlag::Interrupt) {
            self.interrupt(Self::IRQ_VECTOR, 7);
        }
    }

    /// Non-Maskable Interrupt that cannot be disabled. Runs irrespective
    /// of the status register.
    pub fn non_maskable_interrupt_request(&mut self) {
        self.interrupt(Self::NMI_VECTOR, 8)
    }

    fn interrupt(&mut self, pc_location: u16, cycles: u64) {
        // Push PC to Stack
        self.push_stack((self.program_counter & 0x00FF) as u8); // Lo
        self.push_stack((self.program_counter & 0xFF00) as u8); // Hi

        // Push Status Register to Stack
        self.set_flag(&CpuFlag::Break);
        self.set_flag(&CpuFlag::Interrupt);
        self.set_flag(&CpuFlag::Unused);
        self.push_stack(self.status_register);

        // Go to Interrupt Vector
        self.absolute_addr = pc_location;
        let lo = self.read(self.absolute_addr) as u16;
        let hi = self.read(self.absolute_addr + 1) as u16;
        self.program_counter = (hi << 8) | lo;

        self.clock.set_cycles(cycles);
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

    pub fn get_registers(&self) -> Registers {
        Registers {
            a: self.a_register,
            x: self.x_register,
            y: self.y_register,
            status: self.status_register,
            stack_pointer: self.stack_pointer,
            program_counter: self.program_counter,
        }
    }

    /// Trace current cpu state in nestest.log format
    /// Example output line:
    /// ```
    /// C000  4C F5 C5  JMP $C5F5                       A:00 X:00 Y:00 P:24 SP:FD PPU:  0, 21 CYC:7
    /// ```
    ///
    /// Columns:
    /// - `program_counter`: `C000`
    /// - `CPU opcode`: `4C F5 C5` - Variable len - Recall that opcodes are 1-3 bytes. In the case
    /// of shorter opcodes, we keep the columns spacing consistent and left-align the text
    /// - `CPU_opcode in ASM` -
    /// - $80 + X = real address = mem value at real address ???
    ///     @ 80 = 0100 = 00
    ///     - first num is mem referenc ethat we get if we apply offsert to the request
    ///       address based on addressing mode
    ///     - then a u16  target fecthed from [0x80..0x81]
    ///     - the content of that address cell
    /// - rest of the cpu registers: A, X, Y, P, SP
    /// - CPU and PPU clock cycles

    pub fn nestest_trace(&mut self) -> String {
        // Allocing = cringe?
        let mut trace = String::with_capacity(92);
        let pad_till_col = |s: &mut String, col: usize| {
            let amount_to_pad = col - s.len();
            s.extend(std::iter::repeat(' ').take(amount_to_pad));
        };

        trace.extend(format!("{:04X}", self.program_counter).chars());
        pad_till_col(&mut trace, 6);
        // Program Counter
        let opcode: OpCode = self.read(self.program_counter).into();

        // todo!("put in the raw opcode");
        pad_till_col(&mut trace, 16);
        // let x = opcode.decompile();

        // C000  4C F5 C5  JMP $C5F5                       A:00 X:00 Y:00 P:24 SP:FD PPU:  0, 21 CYC:7
        todo!("write the trace");
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Registers {
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub status: u8,
    pub stack_pointer: u8,
    pub program_counter: u16,
}

impl Reset for Cpu {
    // Maybe these should be in their own file
    fn reset(&mut self) {
        // Go to reset vector
        self.absolute_addr = Cpu::RESET_VECTOR;
        let lo = self.read(self.absolute_addr) as u16;
        let hi = self.read(self.absolute_addr + 1) as u16;
        self.program_counter = (hi << 8) | lo; //  TODO: make a join function or macro

        // Reset Internals to default
        self.a_register = 0;
        self.x_register = 0;
        self.y_register = 0;
        self.stack_pointer = Cpu::STACK_POINTER_RESET;
        self.status_register = CpuFlag::Unused as u8;

        self.additional_cycle_addrmode = 0;
        self.additional_cycle_operation = 0;
        self.addressing_mode = AddressingMode::IMP;
        self.absolute_addr = 0;
        self.fetched_data = 0;

        // Take 8 cycles to reset but, don't consume a cylce in this func
        // as this gets called between ticks.
        // Maybe use clock.reset differently?
        self.clock.reset();
        self.clock.set_cycles(8);
    }
}
