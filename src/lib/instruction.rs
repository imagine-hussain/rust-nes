use crate::OpCode;

enum AddressingMode { }

pub struct Instruction {
    pub opcode: OpCode,
    pub name: &'static str,
    pub mode: AddressingMode,
}

impl TryFrom<int> {
    
}

