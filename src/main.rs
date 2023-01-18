use eframe::{run_native, NativeOptions};

<<<<<<< Updated upstream
use eframe::{run_native, NativeOptions, AppCreator};
use egui::Context;

use lib::egui::Gui;
=======
<<<<<<< Updated upstream
use lib::{rendering::Render, Nes};
>>>>>>> Stashed changes

pub fn main() {

    let nes: Nes = Nes::default();
    let app = Gui::new(nes);

    let win_options = Gui::default_win_options();

    run_native("Nes-Rs", NativeOptions::default(), Box::new(|cc| {
        Box::new(app)
    }));


=======
use lib::{egui::Gui, Nes};

pub fn main() {
    let nes: Nes = Nes::default();
    let app = Gui::new(nes);

    run_native(
        "Nes-Rs",
        Gui::default_win_options(),
        Box::new(|_cc| Box::new(app)),
    );
>>>>>>> Stashed changes
}
