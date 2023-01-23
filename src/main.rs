use std::{cell::RefCell, rc::Rc};

use lib::{egui::Gui, Cartridge, Nes};

pub fn main() {
    let mut app = Gui::new(Nes::default());
    if let Some(cartridge_location) = std::env::args().collect::<Vec<_>>().get(1) {
        let nes: &mut Nes = &mut app.nes;

        let cartridge_file_contents =
            std::fs::read(cartridge_location).expect("Failed to read cartridge file");
        let cartridge = Cartridge::try_from(cartridge_file_contents)
            .map(|c| Rc::new(RefCell::new(c)))
            .expect("Failed to load cartridge");
        nes.insert_cartidge(Some(cartridge));
    }

    eframe::run_native(
        "Nes-Rs",
        Gui::default_win_options(),
        Box::new(|_cc| Box::new(app)),
    );
}
