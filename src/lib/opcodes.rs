
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
impl From<u8> for OpCode {

    fn from(raw: u8) -> Self {
        let (msd, lsd) = (raw >> 4, raw & 0x0F);
        match (lsd, msd) {
            // HI-0:
            (0x0, 0x0) => OpCode::new(BRK, IMM, 7),
            (0x0, 0x1) => OpCode::new(ORA, IZX, 6),
            (0x0, 0x2) => OpCode::new(XXX, IMP, 2),
            (0x0, 0x3) => OpCode::new(XXX, IMP, 8),
            (0x0, 0x4) => OpCode::new(NOP, IMP, 3),
            (0x0, 0x5) => OpCode::new(ORA, ZP0, 3),
            (0x0, 0x6) => OpCode::new(ASL, ZP0, 5),
            (0x0, 0x7) => OpCode::new(XXX, IMP, 5),
            (0x0, 0x8) => OpCode::new(PHP, IMP, 3),
            (0x0, 0x9) => OpCode::new(ORA, IMM, 2),
            (0x0, 0xA) => OpCode::new(ASL, IMP, 2),
            (0x0, 0xB) => OpCode::new(XXX, IMP, 2),
            (0x0, 0xC) => OpCode::new(NOP, IMP, 4),
            (0x0, 0xD) => OpCode::new(ORA, ABS, 4),
            (0x0, 0xE) => OpCode::new(ASL, ABS, 6),
            (0x0, 0xF) => OpCode::new(XXX, IMP, 6),

            // HI-1:
            (0x1, 0x0) => OpCode::new(BPL, REL, 2),
            (0x1, 0x1) => OpCode::new(ORA, IZY, 5),
            (0x1, 0x2) => OpCode::new(XXX, IMP, 2),
            (0x1, 0x3) => OpCode::new(XXX, IMP, 8),
            (0x1, 0x4) => OpCode::new(NOP, IMP, 4),
            (0x1, 0x5) => OpCode::new(ORA, ZPX, 4),
            (0x1, 0x6) => OpCode::new(ASL, ZPX, 6),
            (0x1, 0x7) => OpCode::new(XXX, IMP, 6),
            (0x1, 0x8) => OpCode::new(CLC, IMP, 2),
            (0x1, 0x9) => OpCode::new(ORA, ABY, 4),
            (0x1, 0xA) => OpCode::new(NOP, IMP, 2),
            (0x1, 0xB) => OpCode::new(XXX, IMP, 7),
            (0x1, 0xC) => OpCode::new(NOP, IMP, 4),
            (0x1, 0xD) => OpCode::new(ORA, ABX, 4),
            (0x1, 0xE) => OpCode::new(ASL, ABX, 7),
            (0x1, 0xF) => OpCode::new(XXX, IMP, 7),

            // HI-2:
            (0x2, 0x0) => OpCode::new(JSR, ABS, 6),
            (0x2, 0x1) => OpCode::new(AND, IZX, 6),
            (0x2, 0x2) => OpCode::new(XXX, IMP, 2),
            (0x2, 0x3) => OpCode::new(XXX, IMP, 8),
            (0x2, 0x4) => OpCode::new(BIT, ZP0, 3),
            (0x2, 0x5) => OpCode::new(AND, ZP0, 3),
            (0x2, 0x6) => OpCode::new(ROL, ZP0, 5),
            (0x2, 0x7) => OpCode::new(XXX, IMP, 5),
            (0x2, 0x8) => OpCode::new(PLP, IMP, 4),
            (0x2, 0x9) => OpCode::new(AND, IMM, 2),
            (0x2, 0xA) => OpCode::new(ROL, IMP, 2),
            (0x2, 0xB) => OpCode::new(XXX, IMP, 2),
            (0x2, 0xC) => OpCode::new(BIT, ABS, 4),
            (0x2, 0xD) => OpCode::new(AND, ABS, 4),
            (0x2, 0xE) => OpCode::new(ROL, ABS, 6),
            (0x2, 0xF) => OpCode::new(XXX, IMP, 6),

            // HI-3:
            (0x3, 0x0) => OpCode::new(BMI, REL, 2),
            (0x3, 0x1) => OpCode::new(AND, IZY, 5),
            (0x3, 0x2) => OpCode::new(XXX, IMP, 2),
            (0x3, 0x3) => OpCode::new(XXX, IMP, 8),
            (0x3, 0x4) => OpCode::new(NOP, IMP, 4),
            (0x3, 0x5) => OpCode::new(AND, ZPX, 4),
            (0x3, 0x6) => OpCode::new(ROL, ZPX, 6),
            (0x3, 0x7) => OpCode::new(XXX, IMP, 6),
            (0x3, 0x8) => OpCode::new(SEC, IMP, 2),
            (0x3, 0x9) => OpCode::new(AND, ABY, 4),
            (0x3, 0xA) => OpCode::new(NOP, IMP, 2),
            (0x3, 0xB) => OpCode::new(XXX, IMP, 7),
            (0x3, 0xC) => OpCode::new(NOP, IMP, 4),
            (0x3, 0xD) => OpCode::new(AND, ABX, 4),
            (0x3, 0xE) => OpCode::new(ROL, ABX, 7),
            (0x3, 0xF) => OpCode::new(XXX, IMP, 7),

            // HI-4:
            (0x4, 0x0) => OpCode::new(RTI, IMP, 6),
            (0x4, 0x1) => OpCode::new(EOR, IZX, 6),
            (0x4, 0x2) => OpCode::new(XXX, IMP, 2),
            (0x4, 0x3) => OpCode::new(XXX, IMP, 8),
            (0x4, 0x4) => OpCode::new(NOP, IMP, 3),
            (0x4, 0x5) => OpCode::new(EOR, ZP0, 3),
            (0x4, 0x6) => OpCode::new(LSR, ZP0, 5),
            (0x4, 0x7) => OpCode::new(XXX, IMP, 5),
            (0x4, 0x8) => OpCode::new(PHA, IMP, 3),
            (0x4, 0x9) => OpCode::new(EOR, IMM, 2),
            (0x4, 0xA) => OpCode::new(LSR, IMP, 2),
            (0x4, 0xB) => OpCode::new(XXX, IMP, 2),
            (0x4, 0xC) => OpCode::new(JMP, ABS, 3),
            (0x4, 0xD) => OpCode::new(EOR, ABS, 4),
            (0x4, 0xE) => OpCode::new(LSR, ABS, 6),
            (0x4, 0xF) => OpCode::new(XXX, IMP, 6),

            // HI-5:
            (0x5, 0x0) => OpCode::new(BVC, REL, 2),
            (0x5, 0x1) => OpCode::new(EOR, IZY, 5),
            (0x5, 0x2) => OpCode::new(XXX, IMP, 2),
            (0x5, 0x3) => OpCode::new(XXX, IMP, 8),
            (0x5, 0x4) => OpCode::new(NOP, IMP, 4),
            (0x5, 0x5) => OpCode::new(EOR, ZPX, 4),
            (0x5, 0x6) => OpCode::new(LSR, ZPX, 6),
            (0x5, 0x7) => OpCode::new(XXX, IMP, 6),
            (0x5, 0x8) => OpCode::new(CLI, IMP, 2),
            (0x5, 0x9) => OpCode::new(EOR, ABY, 4),
            (0x5, 0xA) => OpCode::new(NOP, IMP, 2),
            (0x5, 0xB) => OpCode::new(XXX, IMP, 7),
            (0x5, 0xC) => OpCode::new(NOP, IMP, 4),
            (0x5, 0xD) => OpCode::new(EOR, ABX, 4),
            (0x5, 0xE) => OpCode::new(LSR, ABX, 7),
            (0x5, 0xF) => OpCode::new(XXX, IMP, 7),

            // HI-6:
            (0x6, 0x0) => OpCode::new(RTS, IMP, 6),
            (0x6, 0x1) => OpCode::new(ADC, IZX, 6),
            (0x6, 0x2) => OpCode::new(XXX, IMP, 2),
            (0x6, 0x3) => OpCode::new(XXX, IMP, 8),
            (0x6, 0x4) => OpCode::new(NOP, IMP, 3),
            (0x6, 0x5) => OpCode::new(ADC, ZP0, 3),
            (0x6, 0x6) => OpCode::new(ROR, ZP0, 5),
            (0x6, 0x7) => OpCode::new(XXX, IMP, 5),
            (0x6, 0x8) => OpCode::new(PLA, IMP, 4),
            (0x6, 0x9) => OpCode::new(ADC, IMM, 2),
            (0x6, 0xA) => OpCode::new(ROR, IMP, 2),
            (0x6, 0xB) => OpCode::new(XXX, IMP, 2),
            (0x6, 0xC) => OpCode::new(JMP, IND, 5),
            (0x6, 0xD) => OpCode::new(ADC, ABS, 4),
            (0x6, 0xE) => OpCode::new(ROR, ABS, 6),
            (0x6, 0xF) => OpCode::new(XXX, IMP, 6),

            // HI-7:
            (0x7, 0x0) => OpCode::new(BVS, REL, 2),
            (0x7, 0x1) => OpCode::new(ADC, IZY, 5),
            (0x7, 0x2) => OpCode::new(XXX, IMP, 2),
            (0x7, 0x3) => OpCode::new(XXX, IMP, 8),
            (0x7, 0x4) => OpCode::new(NOP, IMP, 4),
            (0x7, 0x5) => OpCode::new(ADC, ZPX, 4),
            (0x7, 0x6) => OpCode::new(ROR, ZPX, 6),
            (0x7, 0x7) => OpCode::new(XXX, IMP, 6),
            (0x7, 0x8) => OpCode::new(SEI, IMP, 2),
            (0x7, 0x9) => OpCode::new(ADC, ABY, 4),
            (0x7, 0xA) => OpCode::new(NOP, IMP, 2),
            (0x7, 0xB) => OpCode::new(XXX, IMP, 7),
            (0x7, 0xC) => OpCode::new(NOP, IMP, 4),
            (0x7, 0xD) => OpCode::new(ADC, ABX, 4),
            (0x7, 0xE) => OpCode::new(ROR, ABX, 7),
            (0x7, 0xF) => OpCode::new(XXX, IMP, 7),

            // HI-8:
            (0x8, 0x0) => OpCode::new(NOP, IMP, 2),
            (0x8, 0x1) => OpCode::new(STA, IZX, 6),
            (0x8, 0x2) => OpCode::new(NOP, IMP, 2),
            (0x8, 0x3) => OpCode::new(XXX, IMP, 6),
            (0x8, 0x4) => OpCode::new(STY, ZP0, 3),
            (0x8, 0x5) => OpCode::new(STA, ZP0, 3),
            (0x8, 0x6) => OpCode::new(STX, ZP0, 3),
            (0x8, 0x7) => OpCode::new(XXX, IMP, 3),
            (0x8, 0x8) => OpCode::new(DEY, IMP, 2),
            (0x8, 0x9) => OpCode::new(NOP, IMP, 2),
            (0x8, 0xA) => OpCode::new(TXA, IMP, 2),
            (0x8, 0xB) => OpCode::new(XXX, IMP, 2),
            (0x8, 0xC) => OpCode::new(STY, ABS, 4),
            (0x8, 0xD) => OpCode::new(STA, ABS, 4),
            (0x8, 0xE) => OpCode::new(STX, ABS, 4),
            (0x8, 0xF) => OpCode::new(XXX, IMP, 4),

            // HI-9:
            (0x9, 0x0) => OpCode::new(BCC, REL, 2),
            (0x9, 0x1) => OpCode::new(STA, IZY, 6),
            (0x9, 0x2) => OpCode::new(XXX, IMP, 2),
            (0x9, 0x3) => OpCode::new(XXX, IMP, 6),
            (0x9, 0x4) => OpCode::new(STY, ZPX, 4),
            (0x9, 0x5) => OpCode::new(STA, ZPX, 4),
            (0x9, 0x6) => OpCode::new(STX, ZPY, 4),
            (0x9, 0x7) => OpCode::new(XXX, IMP, 4),
            (0x9, 0x8) => OpCode::new(TYA, IMP, 2),
            (0x9, 0x9) => OpCode::new(STA, ABY, 5),
            (0x9, 0xA) => OpCode::new(TXS, IMP, 2),
            (0x9, 0xB) => OpCode::new(XXX, IMP, 5),
            (0x9, 0xC) => OpCode::new(NOP, IMP, 5),
            (0x9, 0xD) => OpCode::new(STA, ABX, 5),
            (0x9, 0xE) => OpCode::new(XXX, IMP, 5),
            (0x9, 0xF) => OpCode::new(XXX, IMP, 5),

            // HI-A:
            (0xA, 0x0) => OpCode::new(LDY, IMM, 2),
            (0xA, 0x1) => OpCode::new(LDA, IZX, 6),
            (0xA, 0x2) => OpCode::new(LDX, IMM, 2),
            (0xA, 0x3) => OpCode::new(XXX, IMP, 6),
            (0xA, 0x4) => OpCode::new(LDY, ZP0, 3),
            (0xA, 0x5) => OpCode::new(LDA, ZP0, 3),
            (0xA, 0x6) => OpCode::new(LDX, ZP0, 3),
            (0xA, 0x7) => OpCode::new(XXX, IMP, 3),
            (0xA, 0x8) => OpCode::new(TAY, IMP, 2),
            (0xA, 0x9) => OpCode::new(LDA, IMM, 2),
            (0xA, 0xA) => OpCode::new(TAX, IMP, 2),
            (0xA, 0xB) => OpCode::new(XXX, IMP, 2),
            (0xA, 0xC) => OpCode::new(LDY, ABS, 4),
            (0xA, 0xD) => OpCode::new(LDA, ABS, 4),
            (0xA, 0xE) => OpCode::new(LDX, ABS, 4),
            (0xA, 0xF) => OpCode::new(XXX, IMP, 4),

            // HI-B:
            (0xB, 0x0) => OpCode::new(BCS, REL, 2),
            (0xB, 0x1) => OpCode::new(LDA, IZY, 5),
            (0xB, 0x2) => OpCode::new(XXX, IMP, 2),
            (0xB, 0x3) => OpCode::new(XXX, IMP, 5),
            (0xB, 0x4) => OpCode::new(LDY, ZPX, 4),
            (0xB, 0x5) => OpCode::new(LDA, ZPX, 4),
            (0xB, 0x6) => OpCode::new(LDX, ZPY, 4),
            (0xB, 0x7) => OpCode::new(XXX, IMP, 4),
            (0xB, 0x8) => OpCode::new(CLV, IMP, 2),
            (0xB, 0x9) => OpCode::new(LDA, ABY, 4),
            (0xB, 0xA) => OpCode::new(TSX, IMP, 2),
            (0xB, 0xB) => OpCode::new(XXX, IMP, 4),
            (0xB, 0xC) => OpCode::new(LDY, ABX, 4),
            (0xB, 0xD) => OpCode::new(LDA, ABX, 4),
            (0xB, 0xE) => OpCode::new(LDX, ABY, 4),
            (0xB, 0xF) => OpCode::new(XXX, IMP, 4),

            // HI-C:
            (0xC, 0x0) => OpCode::new(CPY, IMM, 2),
            (0xC, 0x1) => OpCode::new(CMP, IZX, 6),
            (0xC, 0x2) => OpCode::new(NOP, IMP, 2),
            (0xC, 0x3) => OpCode::new(XXX, IMP, 8),
            (0xC, 0x4) => OpCode::new(CPY, ZP0, 3),
            (0xC, 0x5) => OpCode::new(CMP, ZP0, 3),
            (0xC, 0x6) => OpCode::new(DEC, ZP0, 5),
            (0xC, 0x7) => OpCode::new(XXX, IMP, 5),
            (0xC, 0x8) => OpCode::new(INY, IMP, 2),
            (0xC, 0x9) => OpCode::new(CMP, IMM, 2),
            (0xC, 0xA) => OpCode::new(DEX, IMP, 2),
            (0xC, 0xB) => OpCode::new(XXX, IMP, 2),
            (0xC, 0xC) => OpCode::new(CPY, ABS, 4),
            (0xC, 0xD) => OpCode::new(CMP, ABS, 4),
            (0xC, 0xE) => OpCode::new(DEC, ABS, 6),
            (0xC, 0xF) => OpCode::new(XXX, IMP, 6),

            // HI-D:
            (0xD, 0x0) => OpCode::new(BNE, REL, 2),
            (0xD, 0x1) => OpCode::new(CMP, IZY, 5),
            (0xD, 0x2) => OpCode::new(XXX, IMP, 2),
            (0xD, 0x3) => OpCode::new(XXX, IMP, 8),
            (0xD, 0x4) => OpCode::new(NOP, IMP, 4),
            (0xD, 0x5) => OpCode::new(CMP, ZPX, 4),
            (0xD, 0x6) => OpCode::new(DEC, ZPX, 6),
            (0xD, 0x7) => OpCode::new(XXX, IMP, 6),
            (0xD, 0x8) => OpCode::new(CLD, IMP, 2),
            (0xD, 0x9) => OpCode::new(CMP, ABY, 4),
            (0xD, 0xA) => OpCode::new(NOP, IMP, 2),
            (0xD, 0xB) => OpCode::new(XXX, IMP, 7),
            (0xD, 0xC) => OpCode::new(NOP, IMP, 4),
            (0xD, 0xD) => OpCode::new(CMP, ABX, 4),
            (0xD, 0xE) => OpCode::new(DEC, ABX, 7),
            (0xD, 0xF) => OpCode::new(XXX, IMP, 7),

            // HI-E:
            (0xE, 0x0) => OpCode::new(CPX, IMM, 2),
            (0xE, 0x1) => OpCode::new(SBC, IZX, 6),
            (0xE, 0x2) => OpCode::new(NOP, IMP, 2),
            (0xE, 0x3) => OpCode::new(XXX, IMP, 8),
            (0xE, 0x4) => OpCode::new(CPX, ZP0, 3),
            (0xE, 0x5) => OpCode::new(SBC, ZP0, 3),
            (0xE, 0x6) => OpCode::new(INC, ZP0, 5),
            (0xE, 0x7) => OpCode::new(XXX, IMP, 5),
            (0xE, 0x8) => OpCode::new(INX, IMP, 2),
            (0xE, 0x9) => OpCode::new(SBC, IMM, 2),
            (0xE, 0xA) => OpCode::new(NOP, IMP, 2),
            (0xE, 0xB) => OpCode::new(SBC, IMP, 2),
            (0xE, 0xC) => OpCode::new(CPX, ABS, 4),
            (0xE, 0xD) => OpCode::new(SBC, ABS, 4),
            (0xE, 0xE) => OpCode::new(INC, ABS, 6),
            (0xE, 0xF) => OpCode::new(XXX, IMP, 6),

            // HI-F:
            (0xF, 0x0) => OpCode::new(BEQ, REL, 2),
            (0xF, 0x1) => OpCode::new(SBC, IZY, 5),
            (0xF, 0x2) => OpCode::new(XXX, IMP, 2),
            (0xF, 0x3) => OpCode::new(XXX, IMP, 8),
            (0xF, 0x4) => OpCode::new(NOP, IMP, 4),
            (0xF, 0x5) => OpCode::new(SBC, ZPX, 4),
            (0xF, 0x6) => OpCode::new(INC, ZPX, 6),
            (0xF, 0x7) => OpCode::new(XXX, IMP, 6),
            (0xF, 0x8) => OpCode::new(SED, IMP, 2),
            (0xF, 0x9) => OpCode::new(SBC, ABY, 4),
            (0xF, 0xA) => OpCode::new(NOP, IMP, 2),
            (0xF, 0xB) => OpCode::new(XXX, IMP, 7),
            (0xF, 0xC) => OpCode::new(NOP, IMP, 4),
            (0xF, 0xD) => OpCode::new(SBC, ABX, 4),
            (0xF, 0xE) => OpCode::new(INC, ABX, 7),
            (0xF, 0xF) => OpCode::new(XXX, IMP, 7),

            // (0x1, 0x0) => OpCode::new(code_type, addressing_mode, cycles),
            // (0x1, 0x0) => OpCode::new(code_type, addressing_mode, cycles),
            // (0x1, 0x0) => OpCode::new(code_type, addressing_mode, cycles),
            // (0x1, 0x0) => OpCode::new(code_type, addressing_mode, cycles),
            // (0x1, 0x0) => OpCode::new(code_type, addressing_mode, cycles),
            // (0x1, 0x0) => OpCode::new(code_type, addressing_mode, cycles),
            // (0x1, 0x0) => OpCode::new(code_type, addressing_mode, cycles),
            // (0x1, 0x0) => OpCode::new(code_type, addressing_mode, cycles),
            // (0x1, 0x0) => OpCode::new(code_type, addressing_mode, cycles),
            // (0x1, 0x0) => OpCode::new(code_type, addressing_mode, cycles),
            // (0x1, 0x0) => OpCode::new(code_type, addressing_mode, cycles),
            // (0x1, 0x0) => OpCode::new(code_type, addressing_mode, cycles),
            // (0x1, 0x0) => OpCode::new(code_type, addressing_mode, cycles),
            // (0x1, 0x0) => OpCode::new(code_type, addressing_mode, cycles),
            _ => OpCode::new(XXX, IMP, 0),
        }
    }
}


pub enum OpCodeType {
    ADC,
    AND,
    ASL,
    BCC,
    BCS,
    BEQ,
    BIT,
    BMI,
    BNE,
    BPL,
    BRK,
    BVC,
    BVS,
    CLC,
    CLD,
    CLI,
    CLV,
    CMP,
    CPX,
    CPY,
    DEC,
    DEX,
    DEY,
    EOR,
    INC,
    INX,
    INY,
    JMP,
    JSR,
    LDA,
    LDX,
    LDY,
    LSR,
    NOP,
    ORA,
    PHA,
    PHP,
    PLA,
    PLP,
    ROL,
    ROR,
    RTI,
    RTS,
    SBC,
    SEC,
    SED,
    SEI,
    STA,
    STX,
    STY,
    TAX,
    TAY,
    TSX,
    TXA,
    TXS,
    TYA,
    /// Illegal; Not a real thing in the actual NES. Anything not implemented is illegal
    XXX,
}

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

