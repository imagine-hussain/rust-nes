use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;
use std::time::Instant;

use eframe::App;
use eframe::NativeOptions;
use egui::CentralPanel;
use egui::Context;
use egui::SidePanel;
use egui::Ui;
use egui_file::FileDialog;
use fstrings::f;
use fstrings::format_args_f;

use thousands::Separable;

use crate::cpu::cpu::Registers;
use crate::Cartridge;
use crate::Clock;
use crate::Nes;
use crate::RcCell;
use crate::Reset;

pub struct Gui {
    pub nes: Nes,
    clock: Clock,
    startup_time: Instant,
    last_frame: Instant,
    framerate: u32,
    open_file_dialog: Option<FileDialog>,
    opened_file: Option<PathBuf>,
    playback_speed: Option<f64>,
}

impl Gui {
    const FRAMERATE_UPDATE_INTERVAL: u64 = 10;

    pub fn new(nes: Nes) -> Self {
        Self {
            nes,
            clock: Clock::default(),
            startup_time: Instant::now(),
            last_frame: Instant::now(),
            framerate: 0,
            open_file_dialog: None,
            opened_file: None,
            playback_speed: None,
        }
    }

    pub fn default_win_options() -> NativeOptions {
        NativeOptions {
            ..Default::default()
        }
    }

    fn debug_frames_cycles(&mut self, ui: &mut Ui) {
        egui::CollapsingHeader::new("Frames and Cycles").show(ui, |ui| {
            let framerate = self.framerate;
            let framerate_str = f!("Framerate: {framerate}");

            let cycles_count = self.nes.cpu_ref().clock.total_ticks().separate_with_commas();
            let cycles_str = f!("Cycles: {cycles_count}");

            let tick_number = self.clock.total_ticks();
            let frame_str = fstrings::f!("Frame: {tick_number}");

            let elapsed_time = self.startup_time.elapsed().as_secs_f32();
            let elapsed_str = fstrings::f!("Elapsed Time: {elapsed_time:.4}s");
            if tick_number % Self::FRAMERATE_UPDATE_INTERVAL == 0 {
                self.update_framerate();
                self.update_delta_time();
            }

            ui.separator();
            ui.label(frame_str);
            ui.separator();
            ui.label(framerate_str);
            ui.separator();
            ui.label(elapsed_str);
            ui.separator();
            ui.label(cycles_str);
        });
    }

    fn debug_registers(ui: &mut Ui, registers: Registers) {
        egui::CollapsingHeader::new("Registers").show(ui, |ui| {
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
        });
    }

    fn simulate_nes_frame(&mut self) {
        // Each frame is exactly 33277.5 frames; need to alternate
        let cycles = match self.clock.total_ticks() % 2 == 0 {
            true => 33278,
            false => 33278,
        } as f64 * self.playback_speed.unwrap_or(1.0);
        for _ in 0..cycles.round() as u64 {
            self.nes.tick()
        }
    }

    fn update_delta_time(&mut self) {
        self.last_frame = Instant::now();
    }

    fn update_framerate(&mut self) -> u32 {
        let delta_time = self.last_frame.elapsed().as_millis();
        self.framerate = match delta_time == 0 {
            true => 69,
            false => ((Self::FRAMERATE_UPDATE_INTERVAL as u128 * 1_000) / delta_time) as u32,
        };
        self.framerate
    }

    fn playback_speed(&self) -> f64 {
        self.playback_speed.unwrap_or(1.0)
    }

}

impl App for Gui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.clock.tick();
        self.simulate_nes_frame();




        SidePanel::right("Debug").show(ctx, |ui| {
            ui.heading("Debug Panel");
            ui.separator();
            self.debug_frames_cycles(ui);
            ui.separator();
            Self::debug_registers(ui, self.nes.cpu_ref().get_registers());
            ui.separator();
            Self::cartridge_info(ui, self.nes.cartridge_ref(), &self.opened_file);
            ui.separator();
        });

        SidePanel::left("Toolbar").show(ctx, |ui| {
            self.render_toolbar(ctx, ui);
        });

        CentralPanel::default().show(ctx, |_ui: &mut egui::Ui| {
            // TODO: put the nes image herecargo run --profile=release-lto
        });
        // force refresh
        ctx.request_repaint();
    }
}

impl Gui {
    fn cartridge_info(ui: &mut Ui, cartridge: Option<RcCell<Cartridge>>, opened_file: &Option<PathBuf>) {
        egui::CollapsingHeader::new("Cartridge Info").show(ui, |ui| {
            let c = match cartridge {
                Some(c) => c,
                None => {
                    ui.label("No cartridge loaded");
                    return;
                }
            };
            let c = c.borrow();

            let header = c.header;
            let program_rom_size = header.prg_rom_size;
            let program_chr_size = header.prg_chr_size;
            let mapper = c.mapper_id;
            let flag_6 = header.flag_6;
            let flag_7 = header.flag_7;

            let program_rom_size_str = f!("Program ROM Size: {program_rom_size} chunks");
            let program_chr_size_str = f!("Program CHR Size: {program_chr_size} chunks");
            let flag_6_str = f!("Flag 6: {flag_6:#04x}");
            let flag_7_str = f!("Flag 7: {flag_7:#04x}");
            let mapper_str = f!("Mapper: {mapper}");

            let open_file_str = match opened_file {
                Some(path) => path.to_str().unwrap_or(""),
                None => "None",
            };
            ui.label(open_file_str);
            ui.add_space(3.0);
            ui.label(program_rom_size_str);
            ui.label(program_chr_size_str);
            ui.label(flag_6_str);
            ui.label(flag_7_str);
            ui.label(mapper_str);
        });
    }

    fn render_toolbar(&mut self, ctx: &Context, ui: &mut Ui) {
        ui.heading("Toolbar");
        ui.separator();
        if ui.button("Reset").clicked() {
            self.nes.reset();
            self.clock.reset();
        }
        if (ui.button("Open")).clicked() {
            let mut dialog = FileDialog::open_file(self.opened_file.clone());
            dialog.open();
            self.open_file_dialog = Some(dialog);
        }

        if let Some(dialog) = &mut self.open_file_dialog {
            if dialog.show(ctx).selected() {
                if let Some(file) = dialog.path() {
                    self.opened_file = Some(file.clone());
                    let file_contents = match std::fs::read(file) {
                        Ok(contents) => contents,
                        Err(e) => {
                            eprintln!("Error reading file: {}", e);
                            return;
                        }
                    };
                    println!("File contents: {}", file_contents.len());

                    let cartridge = Cartridge::try_from(file_contents)
                        .map(|c| Rc::new(RefCell::new(c)))
                        .expect("Failed to load cartridge");

                    println!("Made cartridge");

                    self.nes.insert_cartidge(Some(cartridge));
                    self.nes.reset();
                    assert!(self.nes.cartridge_ref().is_some());
                }
            }
        }

        if (ui.button("Eject")).clicked() {
            self.nes.insert_cartidge(None);
            self.nes.reset();
            self.opened_file = None;
        }

        ui.heading("Playback Speed");
        let mut playback = self.playback_speed();
        ui.add(egui::widgets::Slider::new(&mut playback, 0.0_f64 ..=3.0_f64));
        self.playback_speed = Some(playback);


        // Scale the framerate to the playback speed
        ui.horizontal(|ui| {
            if (ui.button("Pause")).clicked() {
                self.playback_speed = Some(0.0);
            }
            if (ui.button("Reset")).clicked() {
                self.playback_speed = None;
            }
            if (ui.button("0.5")).clicked() {
                self.playback_speed = Some(0.5);
            }
            if (ui.button("1x")).clicked() {
                self.playback_speed = None;
            }
            if (ui.button("2x")).clicked() {
                self.playback_speed = Some(2.0);
            }
            if (ui.button("3x")).clicked() {
                self.playback_speed = Some(3.0);
            }
        });

    }
}
