mod bus;
mod cartridge;
mod clock;
mod cpu;
mod nes;
mod opcodes;
// TODO: Only export the public API
mod mapper;
pub mod ppu;

pub use bus::Bus;
pub use cartridge::Cartridge;
pub use clock::Clock;
pub use cpu::Cpu;
pub use mapper::Mapper;
pub use nes::Nes;
pub use ppu::Ppu;

use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

pub type RcCell<T> = Rc<RefCell<T>>;
pub type WeakCell<T> = Weak<RefCell<T>>;

pub trait Reset {
    fn reset(&mut self);
}
