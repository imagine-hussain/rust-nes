mod bus;
mod cpu;
mod nes;
mod opcodes;
mod clock;
mod cartridge;
// TODO: Only export the public API
pub mod ppu;

pub use bus::Bus;
pub use cpu::Cpu;
pub use nes::Nes;
pub use clock::Clock;
pub use ppu::Ppu;
pub use cartridge::Cartridge;

use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

pub type RcCell<T> = Rc<RefCell<T>>;
pub type WeakCell<T> = Weak<RefCell<T>>;

pub trait Reset {
    fn reset(&mut self);
}
