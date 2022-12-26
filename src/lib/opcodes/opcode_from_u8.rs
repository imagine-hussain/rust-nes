use super::OpCode;
use crate::cpu::AddressingMode::*;
use crate::opcodes::OpCodeType::*;

impl From<u8> for OpCode {
    fn from(raw: u8) -> Self {
        let (hi, lo) = (raw >> 4, raw & 0x0F);
        match hi {
            0x0 => opcode_from_hi_0x0(lo),
            0x1 => opcode_from_hi_0x1(lo),
            0x2 => opcode_from_hi_0x2(lo),
            0x3 => opcode_from_hi_0x3(lo),
            0x4 => opcode_from_hi_0x4(lo),
            0x5 => opcode_from_hi_0x5(lo),
            0x6 => opcode_from_hi_0x6(lo),
            0x7 => opcode_from_hi_0x7(lo),
            0x8 => opcode_from_hi_0x8(lo),
            0x9 => opcode_from_hi_0x9(lo),
            0xA => opcode_from_hi_0x_a(lo),
            0xB => opcode_from_hi_0x_b(lo),
            0xC => opcode_from_hi_0x_c(lo),
            0xD => opcode_from_hi_0x_d(lo),
            0xE => opcode_from_hi_0x_e(lo),
            0xF => opcode_from_hi_0x_f(lo),
            _ => unreachable!("Hi nibble bounded by 0xF"),
        }
    }
}

fn opcode_from_hi_0x0(lo: u8) -> OpCode {
    match lo {
        0x0 => OpCode::new(BRK, IMM, 7),
        0x1 => OpCode::new(ORA, IZX, 6),
        0x2 => OpCode::new(XXX, IMP, 2),
        0x3 => OpCode::new(XXX, IMP, 8),
        0x4 => OpCode::new(NOP, IMP, 3),
        0x5 => OpCode::new(ORA, ZP0, 3),
        0x6 => OpCode::new(ASL, ZP0, 5),
        0x7 => OpCode::new(XXX, IMP, 5),
        0x8 => OpCode::new(PHP, IMP, 3),
        0x9 => OpCode::new(ORA, IMM, 2),
        0xA => OpCode::new(ASL, IMP, 2),
        0xB => OpCode::new(XXX, IMP, 2),
        0xC => OpCode::new(NOP, IMP, 4),
        0xD => OpCode::new(ORA, ABS, 4),
        0xE => OpCode::new(ASL, ABS, 6),
        0xF => OpCode::new(XXX, IMP, 6),
        _ => unreachable!("lo nibble bounded by 0xF"),
    }
}

fn opcode_from_hi_0x1(lo: u8) -> OpCode {
    match lo {
        0x0 => OpCode::new(BPL, REL, 2),
        0x1 => OpCode::new(ORA, IZY, 5),
        0x2 => OpCode::new(XXX, IMP, 2),
        0x3 => OpCode::new(XXX, IMP, 8),
        0x4 => OpCode::new(NOP, IMP, 4),
        0x5 => OpCode::new(ORA, ZPX, 4),
        0x6 => OpCode::new(ASL, ZPX, 6),
        0x7 => OpCode::new(XXX, IMP, 6),
        0x8 => OpCode::new(CLC, IMP, 2),
        0x9 => OpCode::new(ORA, ABY, 4),
        0xA => OpCode::new(NOP, IMP, 2),
        0xB => OpCode::new(XXX, IMP, 7),
        0xC => OpCode::new(NOP, IMP, 4),
        0xD => OpCode::new(ORA, ABX, 4),
        0xE => OpCode::new(ASL, ABX, 7),
        0xF => OpCode::new(XXX, IMP, 7),
        _ => unreachable!("lo nibble bounded by 0xF"),
    }
}

fn opcode_from_hi_0x2(lo: u8) -> OpCode {
    match lo {
        0x0 => OpCode::new(JSR, ABS, 6),
        0x1 => OpCode::new(AND, IZX, 6),
        0x2 => OpCode::new(XXX, IMP, 2),
        0x3 => OpCode::new(XXX, IMP, 8),
        0x4 => OpCode::new(BIT, ZP0, 3),
        0x5 => OpCode::new(AND, ZP0, 3),
        0x6 => OpCode::new(ROL, ZP0, 5),
        0x7 => OpCode::new(XXX, IMP, 5),
        0x8 => OpCode::new(PLP, IMP, 4),
        0x9 => OpCode::new(AND, IMM, 2),
        0xA => OpCode::new(ROL, IMP, 2),
        0xB => OpCode::new(XXX, IMP, 2),
        0xC => OpCode::new(BIT, ABS, 4),
        0xD => OpCode::new(AND, ABS, 4),
        0xE => OpCode::new(ROL, ABS, 6),
        0xF => OpCode::new(XXX, IMP, 6),
        _ => unreachable!("lo nibble bounded by 0xF"),
    }
}

fn opcode_from_hi_0x3(lo: u8) -> OpCode {
    match lo {
        0x0 => OpCode::new(BMI, REL, 2),
        0x1 => OpCode::new(AND, IZY, 5),
        0x2 => OpCode::new(XXX, IMP, 2),
        0x3 => OpCode::new(XXX, IMP, 8),
        0x4 => OpCode::new(NOP, IMP, 4),
        0x5 => OpCode::new(AND, ZPX, 4),
        0x6 => OpCode::new(ROL, ZPX, 6),
        0x7 => OpCode::new(XXX, IMP, 6),
        0x8 => OpCode::new(SEC, IMP, 2),
        0x9 => OpCode::new(AND, ABY, 4),
        0xA => OpCode::new(NOP, IMP, 2),
        0xB => OpCode::new(XXX, IMP, 7),
        0xC => OpCode::new(NOP, IMP, 4),
        0xD => OpCode::new(AND, ABX, 4),
        0xE => OpCode::new(ROL, ABX, 7),
        0xF => OpCode::new(XXX, IMP, 7),
        _ => unreachable!("lo nibble bounded by 0xF"),
    }
}

fn opcode_from_hi_0x4(lo: u8) -> OpCode {
    match lo {
        0x0 => OpCode::new(RTI, IMP, 6),
        0x1 => OpCode::new(EOR, IZX, 6),
        0x2 => OpCode::new(XXX, IMP, 2),
        0x3 => OpCode::new(XXX, IMP, 8),
        0x4 => OpCode::new(NOP, IMP, 3),
        0x5 => OpCode::new(EOR, ZP0, 3),
        0x6 => OpCode::new(LSR, ZP0, 5),
        0x7 => OpCode::new(XXX, IMP, 5),
        0x8 => OpCode::new(PHA, IMP, 3),
        0x9 => OpCode::new(EOR, IMM, 2),
        0xA => OpCode::new(LSR, IMP, 2),
        0xB => OpCode::new(XXX, IMP, 2),
        0xC => OpCode::new(JMP, ABS, 3),
        0xD => OpCode::new(EOR, ABS, 4),
        0xE => OpCode::new(LSR, ABS, 6),
        0xF => OpCode::new(XXX, IMP, 6),
        _ => unreachable!("lo nibble bounded by 0xF"),
    }
}

fn opcode_from_hi_0x5(lo: u8) -> OpCode {
    match lo {
        0x0 => OpCode::new(BVC, REL, 2),
        0x1 => OpCode::new(EOR, IZY, 5),
        0x2 => OpCode::new(XXX, IMP, 2),
        0x3 => OpCode::new(XXX, IMP, 8),
        0x4 => OpCode::new(NOP, IMP, 4),
        0x5 => OpCode::new(EOR, ZPX, 4),
        0x6 => OpCode::new(LSR, ZPX, 6),
        0x7 => OpCode::new(XXX, IMP, 6),
        0x8 => OpCode::new(CLI, IMP, 2),
        0x9 => OpCode::new(EOR, ABY, 4),
        0xA => OpCode::new(NOP, IMP, 2),
        0xB => OpCode::new(XXX, IMP, 7),
        0xC => OpCode::new(NOP, IMP, 4),
        0xD => OpCode::new(EOR, ABX, 4),
        0xE => OpCode::new(LSR, ABX, 7),
        0xF => OpCode::new(XXX, IMP, 7),
        _ => unreachable!("lo nibble bounded by 0xF"),
    }
}

fn opcode_from_hi_0x6(lo: u8) -> OpCode {
    match lo {
        0x0 => OpCode::new(RTS, IMP, 6),
        0x1 => OpCode::new(ADC, IZX, 6),
        0x2 => OpCode::new(XXX, IMP, 2),
        0x3 => OpCode::new(XXX, IMP, 8),
        0x4 => OpCode::new(NOP, IMP, 3),
        0x5 => OpCode::new(ADC, ZP0, 3),
        0x6 => OpCode::new(ROR, ZP0, 5),
        0x7 => OpCode::new(XXX, IMP, 5),
        0x8 => OpCode::new(PLA, IMP, 4),
        0x9 => OpCode::new(ADC, IMM, 2),
        0xA => OpCode::new(ROR, IMP, 2),
        0xB => OpCode::new(XXX, IMP, 2),
        0xC => OpCode::new(JMP, IND, 5),
        0xD => OpCode::new(ADC, ABS, 4),
        0xE => OpCode::new(ROR, ABS, 6),
        0xF => OpCode::new(XXX, IMP, 6),
        _ => unreachable!("lo nibble bounded by 0xF"),
    }
}

fn opcode_from_hi_0x7(lo: u8) -> OpCode {
    match lo {
        0x0 => OpCode::new(BVS, REL, 2),
        0x1 => OpCode::new(ADC, IZY, 5),
        0x2 => OpCode::new(XXX, IMP, 2),
        0x3 => OpCode::new(XXX, IMP, 8),
        0x4 => OpCode::new(NOP, IMP, 4),
        0x5 => OpCode::new(ADC, ZPX, 4),
        0x6 => OpCode::new(ROR, ZPX, 6),
        0x7 => OpCode::new(XXX, IMP, 6),
        0x8 => OpCode::new(SEI, IMP, 2),
        0x9 => OpCode::new(ADC, ABY, 4),
        0xA => OpCode::new(NOP, IMP, 2),
        0xB => OpCode::new(XXX, IMP, 7),
        0xC => OpCode::new(NOP, IMP, 4),
        0xD => OpCode::new(ADC, ABX, 4),
        0xE => OpCode::new(ROR, ABX, 7),
        0xF => OpCode::new(XXX, IMP, 7),
        _ => unreachable!("lo nibble bounded by 0xF"),
    }
}

fn opcode_from_hi_0x8(lo: u8) -> OpCode {
    match lo {
        0x0 => OpCode::new(NOP, IMP, 2),
        0x1 => OpCode::new(STA, IZX, 6),
        0x2 => OpCode::new(NOP, IMP, 2),
        0x3 => OpCode::new(XXX, IMP, 6),
        0x4 => OpCode::new(STY, ZP0, 3),
        0x5 => OpCode::new(STA, ZP0, 3),
        0x6 => OpCode::new(STX, ZP0, 3),
        0x7 => OpCode::new(XXX, IMP, 3),
        0x8 => OpCode::new(DEY, IMP, 2),
        0x9 => OpCode::new(NOP, IMP, 2),
        0xA => OpCode::new(TXA, IMP, 2),
        0xB => OpCode::new(XXX, IMP, 2),
        0xC => OpCode::new(STY, ABS, 4),
        0xD => OpCode::new(STA, ABS, 4),
        0xE => OpCode::new(STX, ABS, 4),
        0xF => OpCode::new(XXX, IMP, 4),
        _ => unreachable!("lo nibble bounded by 0xF"),
    }
}

fn opcode_from_hi_0x9(lo: u8) -> OpCode {
    match lo {
        0x0 => OpCode::new(BCC, REL, 2),
        0x1 => OpCode::new(STA, IZY, 6),
        0x2 => OpCode::new(XXX, IMP, 2),
        0x3 => OpCode::new(XXX, IMP, 6),
        0x4 => OpCode::new(STY, ZPX, 4),
        0x5 => OpCode::new(STA, ZPX, 4),
        0x6 => OpCode::new(STX, ZPY, 4),
        0x7 => OpCode::new(XXX, IMP, 4),
        0x8 => OpCode::new(TYA, IMP, 2),
        0x9 => OpCode::new(STA, ABY, 5),
        0xA => OpCode::new(TXS, IMP, 2),
        0xB => OpCode::new(XXX, IMP, 5),
        0xC => OpCode::new(NOP, IMP, 5),
        0xD => OpCode::new(STA, ABX, 5),
        0xE => OpCode::new(XXX, IMP, 5),
        0xF => OpCode::new(XXX, IMP, 5),
        _ => unreachable!("lo nibble bounded by 0xF"),
    }
}

fn opcode_from_hi_0x_a(lo: u8) -> OpCode {
    match lo {
        0x0 => OpCode::new(LDY, IMM, 2),
        0x1 => OpCode::new(LDA, IZX, 6),
        0x2 => OpCode::new(LDX, IMM, 2),
        0x3 => OpCode::new(XXX, IMP, 6),
        0x4 => OpCode::new(LDY, ZP0, 3),
        0x5 => OpCode::new(LDA, ZP0, 3),
        0x6 => OpCode::new(LDX, ZP0, 3),
        0x7 => OpCode::new(XXX, IMP, 3),
        0x8 => OpCode::new(TAY, IMP, 2),
        0x9 => OpCode::new(LDA, IMM, 2),
        0xA => OpCode::new(TAX, IMP, 2),
        0xB => OpCode::new(XXX, IMP, 2),
        0xC => OpCode::new(LDY, ABS, 4),
        0xD => OpCode::new(LDA, ABS, 4),
        0xE => OpCode::new(LDX, ABS, 4),
        0xF => OpCode::new(XXX, IMP, 4),
        _ => unreachable!("lo nibble bounded by 0xF"),
    }
}

fn opcode_from_hi_0x_b(lo: u8) -> OpCode {
    match lo {
        0x0 => OpCode::new(BCS, REL, 2),
        0x1 => OpCode::new(LDA, IZY, 5),
        0x2 => OpCode::new(XXX, IMP, 2),
        0x3 => OpCode::new(XXX, IMP, 5),
        0x4 => OpCode::new(LDY, ZPX, 4),
        0x5 => OpCode::new(LDA, ZPX, 4),
        0x6 => OpCode::new(LDX, ZPY, 4),
        0x7 => OpCode::new(XXX, IMP, 4),
        0x8 => OpCode::new(CLV, IMP, 2),
        0x9 => OpCode::new(LDA, ABY, 4),
        0xA => OpCode::new(TSX, IMP, 2),
        0xB => OpCode::new(XXX, IMP, 4),
        0xC => OpCode::new(LDY, ABX, 4),
        0xD => OpCode::new(LDA, ABX, 4),
        0xE => OpCode::new(LDX, ABY, 4),
        0xF => OpCode::new(XXX, IMP, 4),
        _ => unreachable!("lo nibble bounded by 0xF"),
    }
}

fn opcode_from_hi_0x_c(lo: u8) -> OpCode {
    match lo {
        0x0 => OpCode::new(CPY, IMM, 2),
        0x1 => OpCode::new(CMP, IZX, 6),
        0x2 => OpCode::new(NOP, IMP, 2),
        0x3 => OpCode::new(XXX, IMP, 8),
        0x4 => OpCode::new(CPY, ZP0, 3),
        0x5 => OpCode::new(CMP, ZP0, 3),
        0x6 => OpCode::new(DEC, ZP0, 5),
        0x7 => OpCode::new(XXX, IMP, 5),
        0x8 => OpCode::new(INY, IMP, 2),
        0x9 => OpCode::new(CMP, IMM, 2),
        0xA => OpCode::new(DEX, IMP, 2),
        0xB => OpCode::new(XXX, IMP, 2),
        0xC => OpCode::new(CPY, ABS, 4),
        0xD => OpCode::new(CMP, ABS, 4),
        0xE => OpCode::new(DEC, ABS, 6),
        0xF => OpCode::new(XXX, IMP, 6),
        _ => unreachable!("lo nibble bounded by 0xF"),
    }
}

fn opcode_from_hi_0x_d(lo: u8) -> OpCode {
    match lo {
        0x0 => OpCode::new(BNE, REL, 2),
        0x1 => OpCode::new(CMP, IZY, 5),
        0x2 => OpCode::new(XXX, IMP, 2),
        0x3 => OpCode::new(XXX, IMP, 8),
        0x4 => OpCode::new(NOP, IMP, 4),
        0x5 => OpCode::new(CMP, ZPX, 4),
        0x6 => OpCode::new(DEC, ZPX, 6),
        0x7 => OpCode::new(XXX, IMP, 6),
        0x8 => OpCode::new(CLD, IMP, 2),
        0x9 => OpCode::new(CMP, ABY, 4),
        0xA => OpCode::new(NOP, IMP, 2),
        0xB => OpCode::new(XXX, IMP, 7),
        0xC => OpCode::new(NOP, IMP, 4),
        0xD => OpCode::new(CMP, ABX, 4),
        0xE => OpCode::new(DEC, ABX, 7),
        0xF => OpCode::new(XXX, IMP, 7),
        _ => unreachable!("lo nibble bounded by 0xF"),
    }
}

fn opcode_from_hi_0x_e(lo: u8) -> OpCode {
    match lo {
        0x0 => OpCode::new(CPX, IMM, 2),
        0x1 => OpCode::new(SBC, IZX, 6),
        0x2 => OpCode::new(NOP, IMP, 2),
        0x3 => OpCode::new(XXX, IMP, 8),
        0x4 => OpCode::new(CPX, ZP0, 3),
        0x5 => OpCode::new(SBC, ZP0, 3),
        0x6 => OpCode::new(INC, ZP0, 5),
        0x7 => OpCode::new(XXX, IMP, 5),
        0x8 => OpCode::new(INX, IMP, 2),
        0x9 => OpCode::new(SBC, IMM, 2),
        0xA => OpCode::new(NOP, IMP, 2),
        0xB => OpCode::new(SBC, IMP, 2),
        0xC => OpCode::new(CPX, ABS, 4),
        0xD => OpCode::new(SBC, ABS, 4),
        0xE => OpCode::new(INC, ABS, 6),
        0xF => OpCode::new(XXX, IMP, 6),
        _ => unreachable!("lo nibble bounded by 0xF"),
    }
}

fn opcode_from_hi_0x_f(lo: u8) -> OpCode {
    match lo {
        0x0 => OpCode::new(BEQ, REL, 2),
        0x1 => OpCode::new(SBC, IZY, 5),
        0x2 => OpCode::new(XXX, IMP, 2),
        0x3 => OpCode::new(XXX, IMP, 8),
        0x4 => OpCode::new(NOP, IMP, 4),
        0x5 => OpCode::new(SBC, ZPX, 4),
        0x6 => OpCode::new(INC, ZPX, 6),
        0x7 => OpCode::new(XXX, IMP, 6),
        0x8 => OpCode::new(SED, IMP, 2),
        0x9 => OpCode::new(SBC, ABY, 4),
        0xA => OpCode::new(NOP, IMP, 2),
        0xB => OpCode::new(XXX, IMP, 7),
        0xC => OpCode::new(NOP, IMP, 4),
        0xD => OpCode::new(SBC, ABX, 4),
        0xE => OpCode::new(INC, ABX, 7),
        0xF => OpCode::new(XXX, IMP, 7),
        _ => unreachable!("lo nibble bounded by 0xF"),
    }
}
