use std::{cell::RefCell, rc::Rc};

use env_logger::Env;
use lib::{egui::Gui, Cartridge, Nes};
use log::{debug, error, info, trace, warn};

pub fn main() {
    let mut app = Gui::new(Nes::default());
    startup_logger();
    if let Some(cartridge_location) = std::env::args().nth(1) {
        let nes: &mut Nes = &mut app.nes;

        let cartridge_file_contents =
            std::fs::read(cartridge_location).expect("Failed to read cartridge file");
        let cartridge = Cartridge::try_from(cartridge_file_contents)
            .map(|c| Rc::new(RefCell::new(c)))
            .expect("Failed to load cartridge");
        nes.insert_cartidge(Some(cartridge));
    }

    info!("Starting Emulator");
    eframe::run_native(
        "Nes-Rs",
        Gui::default_win_options(),
        Box::new(|_cc| Box::new(app)),
    );
    info!("Exiting now");
}

fn startup_logger() {
    let (default_filter, default_write) = match in_release_build() {
        true => ("info", "always"),
        false => ("debug", "always"),
    };

    let env = Env::default()
        .filter_or("MY_LOG", default_filter)
        .write_style_or("MY_LOG_STYLE", default_write);

    env_logger::init_from_env(env);

    trace!("This is a trace log. If you are running a release build, this should NOT be visible.");
    debug!("This is a debug log. This should only be visible in debug builds.");
    info!("This is an info log. If you are in a release build, you should see [INFO] [WARN] and [ERROR] logs only.");
    warn!("This is a warning log. This should be visible in all builds.");
    error!("This is an error log. If you see this, something has gone horribly wrong.");

    info!("");
    info!("");
    info!("Logger set up successfully!");
    info!("===============================================================");
}

#[cfg(debug_assertions)]
fn in_release_build() -> bool {
    false
}

#[cfg(not(debug_assertions))]
fn in_release_build() -> bool {
    true
}
