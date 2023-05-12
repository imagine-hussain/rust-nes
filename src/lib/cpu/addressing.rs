use self::fetching::*;
use crate::Cpu;

/// Functionality related to addressing modes.
/// Exports an `AddressingMode` enum, where the variants are the different addressing
/// modes. AddressingMode implements a `fetch` method to fetch the value of the
/// operand from the memory, in the given CPU.
/// Goated Resource: <https://www.svaught.com/posts/addr-modes-6502>
#[derive(Debug)]
#[allow(clippy::upper_case_acronyms)]
pub enum AddressingMode {
    /// Implied: No data in the instruction.
    /// This *can* be operating on the accumulator.
    IMP,
    /// Immediate: Data is within the instruction - next byte
    IMM,
    /// Zero Page: Data is within the next byte.
    /// It is within the high byte of the page.
    /// Data is stored as 0xABCD; `0xAB` is the page number
    /// and `0xCD` is the offset within the page.
    /// 256x256 = 65536 bytes
    /// In this address mode, the high byte is always 0.
    ZP0,
    /// Zero Page X Offset: Similar to Zero Page, but with an addition offset inside
    /// the X register
    ZPX,
    /// Zero Page Y Offset: Similar to Zero Page, but with an addition offset inside
    /// the Y register
    ZPY,
    // Relative: The next byte is the offset from the current PC, using
    // signed arithmetic. Used mostly for branching instructions.
    REL,
    /// Absolute: Data is within the next two bytes.
    /// Both the `lo` and `hi` bytes are used to determine the address.
    ABS, // Absolute
    /// Absolute X Offset: Similar to Absolute, but with an addition offset inside
    /// the X register.
    /// If adding the additional offset causes the address to change, then an additional
    /// clock cycle is used.
    ABX,
    /// Absolute Y Offset: Similar to Absolute, but with an addition offset inside
    /// the Y register
    /// If adding the additional offset causes the address to change, then an additional
    /// clock cycle is used.
    ABY,
    /// Indirect: Equivalent to using pointers.
    /// The next two bytes contain a pointer to some other address. The
    /// The data at that target location is used.
    /// This mode is prone to over-flowing into the next page. This is a bug
    /// in the hardware that does not get fixed. <https://nesdev.com/6502bugs.txt>
    IND,
    /// Indirect X Offset: Similar to Indirect, but with an addition offset inside
    /// the X register.
    /// The offset is relative to the zero page.
    IZX,
    /// Indirect Y Offset: Similar to Indirect, but with an addition offset inside
    /// the Y register
    /// The offset is relative to the zero page.
    IZY,
}

impl AddressingMode {
    pub fn fetch(&self) -> fn(&mut Cpu) -> u8 {
        match *self {
            AddressingMode::IMP => fetch_imp,
            AddressingMode::IMM => fetch_imm,
            AddressingMode::ZP0 => fetch_zp0,
            AddressingMode::ZPX => fetch_zpx,
            AddressingMode::ZPY => fetch_zpy,
            AddressingMode::REL => fetch_rel,
            AddressingMode::ABS => fetch_abs,
            AddressingMode::ABX => fetch_abx,
            AddressingMode::ABY => fetch_aby,
            AddressingMode::IND => fetch_ind,
            AddressingMode::IZX => fetch_izx,
            AddressingMode::IZY => fetch_izy,
        }
    }
}

pub(self) mod fetching {
    use crate::Cpu;

    pub(super) fn fetch_imp(cpu: &mut Cpu) -> u8 {
        cpu.fetched_data = cpu.a_register;
        0
    }

    pub(super) fn fetch_imm(cpu: &mut Cpu) -> u8 {
        cpu.absolute_addr += 1;
        0
    }

    pub(super) fn fetch_zp0(cpu: &mut Cpu) -> u8 {
        let offset = cpu.read(cpu.program_counter);

        cpu.absolute_addr = offset as u16;

        0
    }

    pub(super) fn fetch_zpx(cpu: &mut Cpu) -> u8 {
        let offset = cpu.read(cpu.program_counter) + cpu.x_register;
        cpu.absolute_addr = (offset as u16) & 0x00FF;
        cpu.program_counter += 1;
        0
    }

    pub(super) fn fetch_zpy(cpu: &mut Cpu) -> u8 {
        let offset = cpu.read(cpu.program_counter) + cpu.y_register;
        cpu.absolute_addr = (offset as u16) & 0x00FF;
        cpu.program_counter += 1;
        0
    }

    pub(super) fn fetch_rel(cpu: &mut Cpu) -> u8 {
        // TODO: check impl when signed
        cpu.relative_addr = cpu.read(cpu.program_counter) as i8;
        cpu.program_counter += 1;

        0
    }

    pub(super) fn fetch_abs(cpu: &mut Cpu) -> u8 {
        let lo = cpu.read(cpu.program_counter) as u16;
        cpu.program_counter += 1;
        let hi = cpu.read(cpu.program_counter) as u16;
        cpu.program_counter += 1;

        cpu.absolute_addr = (hi << 8) | lo;

        0
    }

    pub(super) fn fetch_abx(cpu: &mut Cpu) -> u8 {
        let lo = cpu.read(cpu.program_counter) as u16;
        cpu.program_counter += 1;
        let hi = cpu.read(cpu.program_counter) as u16;
        cpu.program_counter += 1;

        cpu.absolute_addr = ((hi << 8) | lo) + cpu.x_register as u16;

        // If page overflow, then add a cycle
        match (cpu.absolute_addr & 0xFF00) != (hi << 8) {
            true => 1,
            false => 0,
        }
    }

    pub(super) fn fetch_aby(cpu: &mut Cpu) -> u8 {
        let lo = cpu.read(cpu.program_counter) as u16;
        cpu.program_counter += 1;
        let hi = cpu.read(cpu.program_counter) as u16;
        cpu.program_counter += 1;

        cpu.absolute_addr = ((hi << 8) | lo) + cpu.y_register as u16;

        // If page overflow, then add a cycle
        // (cpu.absolute_addr & 0xFF00) != (hi << 8)).into()
        ((cpu.absolute_addr & 0xFF00) != (hi << 8)).into()
    }
    pub(super) fn fetch_ind(cpu: &mut Cpu) -> u8 {
        let lo = cpu.read(cpu.program_counter) as u16;
        cpu.program_counter += 1;
        let hi = cpu.read(cpu.program_counter) as u16;
        let ptr_addr: u16 = (hi << 8) | lo;

        // Simulate page boundary hardware bug
        cpu.absolute_addr = match lo == 0x00FF {
            // Bug: Lo byte of pointer is 0x00FF.
            // Getting Hi bytes of pointer is at ptr_addr + 1, which is crossing a
            // page boundary. Instead, just wrap around to the start of the same
            // page
            true => ((cpu.read(ptr_addr & 0xFF00) as u16) << 8) | (cpu.read(ptr_addr) as u16),
            // Normal behaviour
            false => ((cpu.read(ptr_addr + 1) as u16) << 8) | (cpu.read(ptr_addr) as u16),
        };

        0
    }

    pub(super) fn fetch_izx(cpu: &mut Cpu) -> u8 {
        let offset: u16 = (cpu.read(cpu.program_counter) + cpu.x_register) as u16;
        cpu.program_counter += 1;

        // & 0x00FF to wrap around instead of moving to the next page
        let lo = cpu.read(offset & 0x00FF) as u16;
        let hi = cpu.read((offset + 1) & 0x00FF) as u16;

        cpu.absolute_addr = (hi << 8) | lo;

        0
    }

    pub(super) fn fetch_izy(cpu: &mut Cpu) -> u8 {
        let offset: u16 = cpu.read(cpu.program_counter) as u16;
        cpu.program_counter += 1;

        let lo = cpu.read(offset & 0x00FF) as u16;
        let hi = cpu.read((offset + 1) & 0x00FF) as u16;

        cpu.absolute_addr = ((hi << 8) | lo) + cpu.y_register as u16;

        // If page overflow, then add a cycle
        ((cpu.absolute_addr & 0xFF00) != (hi << 8)).into()
    }
}
