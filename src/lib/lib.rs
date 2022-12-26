mod bus;
mod cpu;
mod nes;
mod opcodes;
mod clock;

pub use bus::Bus;
pub use cpu::Cpu;
pub use nes::Nes;
pub use clock::Clock;

use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

pub type RcCell<T> = Rc<RefCell<T>>;
pub type WeakCell<T> = Weak<RefCell<T>>;
