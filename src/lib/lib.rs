pub mod cpu;

pub mod bus;
pub mod opcodes;
pub mod instruction;

use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

pub use opcodes::{OpCode, AddressingModes};

pub use bus::Bus;
pub use cpu::Cpu;
pub use instruction::Instruction;

pub type RcCell<T> = Rc<RefCell<T>>;
pub type WeakCell<T> = Weak<RefCell<T>>;


