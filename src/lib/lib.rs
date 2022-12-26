mod bus;
mod cpu;
mod opcodes;

pub use bus::Bus;
pub use cpu::Cpu;

// pub mod cpu;
//
// pub mod addressing;
// pub mod bus;
// pub mod opcodes;
//
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};
//
// pub use addressing::AddressingMode;
// pub use opcodes::opcode::OpCode;
//
// pub use bus::Bus;
// pub use cpu::Cpu;
// // pub use instruction::Instruction;
pub type RcCell<T> = Rc<RefCell<T>>;
pub type WeakCell<T> = Weak<RefCell<T>>;
