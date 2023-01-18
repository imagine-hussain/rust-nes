use std::time::Duration;

use eframe::{run_native, NativeOptions, AppCreator};
use egui::Context;

use lib::egui::Gui;

pub fn main() {

    let nes: Nes = Nes::default();
    let app = Gui::new(nes);

    let win_options = Gui::default_win_options();

    run_native("Nes-Rs", NativeOptions::default(), Box::new(|cc| {
        Box::new(app)
    }));


}
