mod bus;
mod cartridge;
mod clock;
mod cpu;
mod nes;
mod opcodes;
mod mappers;

pub mod egui; // fix privacy

pub mod ppu;
pub mod rendering; // Fix exports for this

pub use bus::Bus;
pub use cartridge::Cartridge;
pub use clock::Clock;
pub use cpu::Cpu;
pub use nes::Nes;
pub use ppu::Ppu;
pub use mappers::Mapper;

use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

pub type RcCell<T> = Rc<RefCell<T>>;
pub type WeakCell<T> = Weak<RefCell<T>>;

pub trait Reset {
    fn reset(&mut self);
}
