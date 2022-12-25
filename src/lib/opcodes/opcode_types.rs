use crate::Cpu;

#[derive(Debug)]
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

impl OpCodeType {
    fn executable(&self, cpu: &mut Cpu) -> Box<dyn Fn(&mut Cpu) -> bool> {
        match *self {
            OpCodeType::ADC => todo!(),
            OpCodeType::AND => todo!(),
            OpCodeType::ASL => todo!(),
            OpCodeType::BCC => todo!(),
            OpCodeType::BCS => todo!(),
            OpCodeType::BEQ => todo!(),
            OpCodeType::BIT => todo!(),
            OpCodeType::BMI => todo!(),
            OpCodeType::BNE => todo!(),
            OpCodeType::BPL => todo!(),
            OpCodeType::BRK => todo!(),
            OpCodeType::BVC => todo!(),
            OpCodeType::BVS => todo!(),
            OpCodeType::CLC => todo!(),
            OpCodeType::CLD => todo!(),
            OpCodeType::CLI => todo!(),
            OpCodeType::CLV => todo!(),
            OpCodeType::CMP => todo!(),
            OpCodeType::CPX => todo!(),
            OpCodeType::CPY => todo!(),
            OpCodeType::DEC => todo!(),
            OpCodeType::DEX => todo!(),
            OpCodeType::DEY => todo!(),
            OpCodeType::EOR => todo!(),
            OpCodeType::INC => todo!(),
            OpCodeType::INX => todo!(),
            OpCodeType::INY => todo!(),
            OpCodeType::JMP => todo!(),
            OpCodeType::JSR => todo!(),
            OpCodeType::LDA => todo!(),
            OpCodeType::LDX => todo!(),
            OpCodeType::LDY => todo!(),
            OpCodeType::LSR => todo!(),
            OpCodeType::NOP => todo!(),
            OpCodeType::ORA => todo!(),
            OpCodeType::PHA => todo!(),
            OpCodeType::PHP => todo!(),
            OpCodeType::PLA => todo!(),
            OpCodeType::PLP => todo!(),
            OpCodeType::ROL => todo!(),
            OpCodeType::ROR => todo!(),
            OpCodeType::RTI => todo!(),
            OpCodeType::RTS => todo!(),
            OpCodeType::SBC => todo!(),
            OpCodeType::SEC => todo!(),
            OpCodeType::SED => todo!(),
            OpCodeType::SEI => todo!(),
            OpCodeType::STA => todo!(),
            OpCodeType::STX => todo!(),
            OpCodeType::STY => todo!(),
            OpCodeType::TAX => todo!(),
            OpCodeType::TAY => todo!(),
            OpCodeType::TSX => todo!(),
            OpCodeType::TXA => todo!(),
            OpCodeType::TXS => todo!(),
            OpCodeType::TYA => todo!(),
            OpCodeType::XXX => todo!(),
        }
    }
}
