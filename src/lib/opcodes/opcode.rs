use crate::cpu::AddressingMode;
use crate::opcodes::OpCodeType;

#[derive(Debug)]
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
