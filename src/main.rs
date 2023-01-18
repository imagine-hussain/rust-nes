use lib::{egui::Gui, Nes};

pub fn main() {
    let app = Gui::new(Nes::default());

    eframe::run_native(
        "Nes-Rs",
        Gui::default_win_options(),
        Box::new(|_cc| Box::new(app)),
    );
}
