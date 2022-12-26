mod bus;
mod cpu;
mod nes;
mod opcodes;

pub use bus::Bus;
pub use cpu::Cpu;
pub use nes::Nes;

use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

pub type RcCell<T> = Rc<RefCell<T>>;
pub type WeakCell<T> = Weak<RefCell<T>>;
