use std::time::Instant;

use eframe::App;
use eframe::NativeOptions;
use egui::CentralPanel;
use egui::RichText;
use egui::SidePanel;
use egui::Ui;
use fstrings::f;
use fstrings::format_args_f;

use crate::cpu::cpu::Registers;
use crate::Clock;
use crate::Nes;

pub struct Gui {
    pub nes: Nes,
    clock: Clock,
    startup_time: Instant,
}

impl Gui {
    pub fn new(nes: Nes) -> Self {
        Self {
            nes,
            clock: Clock::default(),
            startup_time: Instant::now(),
        }
    }

    pub fn default_win_options() -> NativeOptions {
        NativeOptions {
            ..Default::default()
        }
    }

    fn debug_registers(ui: &mut Ui, registers: Registers) {
        let Registers {
            a,
            x,
            y,
            status,
            stack_pointer,
            program_counter,
        } = registers;
        let a_str = f!("A:\t{a:#04x}");
        let x_str = f!("X:\t{x:#04x}");
        let y_str = f!("Y:\t{y:#04x}");
        let status_str = f!("Status:\t{status:#04x}");
        let stack_pointer_str = f!("Stack Pointer:\t{stack_pointer:#04x}");
        let program_counter_str = f!("Program Counter:\t{program_counter:#04x}");

        ui.label(a_str);
        ui.label(x_str);
        ui.label(y_str);
        ui.label(stack_pointer_str);
        ui.label(program_counter_str);
        ui.label(status_str);
        ui.separator();
    }
}

impl App for Gui {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.clock.tick();
        let frame_number = self.clock.total_ticks();
        let frame_str = fstrings::f!("Frame: {frame_number}");

        let elapsed_time = self.startup_time.elapsed().as_secs_f32();
        let elapsed_str = fstrings::f!("Elapsed Time: {elapsed_time}s");

        SidePanel::right("Debug").show(ctx, |ui| {
            ui.heading("Debug Panel");
            ui.separator();
            ui.label(frame_str);
            ui.separator();
            ui.label(elapsed_str);
            ui.separator();
            Self::debug_registers(ui, self.nes.cpu_ref().get_registers());
        });

        CentralPanel::default().show(ctx, |ui: &mut egui::Ui| {});
    }
}
