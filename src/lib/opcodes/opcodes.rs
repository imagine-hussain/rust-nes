
pub struct OpCode {
    pub code_type: OpCodeType,
    pub addressing_mode: AddressingMode,
    pub cycles: u8,
}

impl OpCode {
    pub fn new(code_type: OpCodeType, addressing_mode: AddressingMode, cycles: u8) -> Self {
        OpCode {
            code_type,
            addressing_mode,
            cycles,
        }
    }
}

use OpCodeType::*;
use AddressingMode::*;

use crate::Cpu;
// http://archive.6502.org/datasheets/rockwell_r650x_r651x.pdf : page 10

impl From<u8> for OpCodeType {
    fn from(val: u8) -> Self {
        match val {
            0x0 => todo!(),
            _ => OpCodeType::XXX,
        }
    }
}

/// Goated Resource: https://www.svaught.com/posts/addr-modes-6502
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
    /// the X register
    IZX,
    /// Indirect Y Offset: Similar to Indirect, but with an addition offset inside
    /// the Y register
    IZY,
}

impl OpCodeType {
    pub fn addr_mode(&self, arg: &mut Cpu) -> bool {

        todo!()
    }

    pub fn execute(&self, arg: &mut Cpu) -> bool {
        match self {
            ADC => todo!(),
            AND => todo!(),
            ASL => todo!(),
            BCC => todo!(),
            BCS => todo!(),
            BEQ => todo!(),
            BIT => todo!(),
            BMI => todo!(),
            BNE => todo!(),
            BPL => todo!(),
            BRK => todo!(),
            BVC => todo!(),
            BVS => todo!(),
            CLC => todo!(),
            CLD => todo!(),
            CLI => todo!(),
            CLV => todo!(),
            CMP => todo!(),
            CPX => todo!(),
            CPY => todo!(),
            DEC => todo!(),
            DEX => todo!(),
            DEY => todo!(),
            EOR => todo!(),
            INC => todo!(),
            INX => todo!(),
            INY => todo!(),
            JMP => todo!(),
            JSR => todo!(),
            LDA => todo!(),
            LDX => todo!(),
            LDY => todo!(),
            LSR => todo!(),
            NOP => todo!(),
            ORA => todo!(),
            PHA => todo!(),
            PHP => todo!(),
            PLA => todo!(),
            PLP => todo!(),
            ROL => todo!(),
            ROR => todo!(),
            RTI => todo!(),
            RTS => todo!(),
            SBC => todo!(),
            SEC => todo!(),
            SED => todo!(),
            SEI => todo!(),
            STA => todo!(),
            STX => todo!(),
            STY => todo!(),
            TAX => todo!(),
            TAY => todo!(),
            TSX => todo!(),
            TXA => todo!(),
            TXS => todo!(),
            TYA => todo!(),
            XXX => todo!(),
        }
    }
}

