use eframe::run_native;

use lib::{egui::Gui, Nes};

pub fn main() {
    let nes: Nes = Nes::default();
    let app = Gui::new(nes);

    run_native(
        "Nes-Rs",
        Gui::default_win_options(),
        Box::new(|_cc| Box::new(app)),
    );
}
