use std::time::Duration;

use eframe::{run_native, NativeOptions, AppCreator};
use egui::Context;
use lib::{rendering::Render, Nes};

use lib::egui::Gui;

pub fn main() {

    let nes: Nes = Nes::default();
    let app = Gui::new(nes);
    let win_options = NativeOptions::default();
    let win_options = Gui::default_win_options();
    run_native("Nes-Rs", NativeOptions::default(), Box::new(|cc| {
        Box::new(app)
    }));

    // let ctx = Context::default();
    //
    // egui::CentralPanel::default().show(&ctx, |ui| {
    //     ui.add(egui::Label::new("Hello World!"));
    //     ui.label("A shorter and more convenient way to add a label.");
    //     if ui.button("Click me").clicked() {
    //         // take some action here
    //     }
    // });

    // let mut _nes: Nes = Nes::default();
    // let mut r = Render::init().unwrap();

    // 'running: loop {
    //     // Todo: Pipe events to nes
    //     // Todo: connect cartridge
    //     // nes.tick();
    //     if r.tick().is_err() {
    //         break 'running;
    //     }

    //     ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    // }


}
