mod bus;
mod cpu;
mod nes;
mod opcodes;
mod clock;
// TODO: Only export the public API
pub mod ppu;

pub use bus::Bus;
pub use cpu::Cpu;
pub use nes::Nes;
pub use clock::Clock;
pub use ppu::Ppu;

use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

pub type RcCell<T> = Rc<RefCell<T>>;
pub type WeakCell<T> = Weak<RefCell<T>>;
