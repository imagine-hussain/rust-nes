use std::time::Duration;

use lib::{rendering::Render, Nes};

pub fn main() {
    let mut _nes: Nes = Nes::default();
    let mut r = Render::init().unwrap();

    'running: loop {
        // Todo: Pipe events to nes
        // Todo: connect cartridge
        // nes.tick();
        if r.tick().is_err() {
            break 'running;
        }

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

}
