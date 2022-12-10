pub mod cpu;

pub mod bus;
pub mod opcodes;

use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

pub use opcodes::OpCode;

pub use bus::Bus;
pub use cpu::Cpu;

pub type RcCell<T> = Rc<RefCell<T>>;
pub type WeakCell<T> = Weak<RefCell<T>>;


