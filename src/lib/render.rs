use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::{video::WindowBuildError, IntegerOrSdlError, Sdl, VideoSubsystem};
// use std::time::Duration;

pub struct Render {
    pub sdl_context: sdl2::Sdl,
    pub video_subsystem: sdl2::VideoSubsystem,
    pub canvas: sdl2::render::Canvas<sdl2::video::Window>,
    pub event_pump: sdl2::EventPump,
    pub times_polled: u64,
}

impl Render {
    pub fn init() -> Result<Self, RenderError> {
        let sdl_context: Sdl = sdl2::init()?;
        let video_subsystem: VideoSubsystem = sdl_context.video()?;
        let window = video_subsystem
            .window("rust nes", 800, 600)
            .position_centered()
            .build()?;
        let mut canvas = window.into_canvas().build()?;
        let event_pump = sdl_context.event_pump()?;

        canvas.set_draw_color(Color::RGB(0, 255, 255));
        canvas.clear();
        canvas.set_draw_color(Color::RGB(255, 0, 100));
        canvas.fill_rect(Rect::new(0, 0, 100, 150))?;
        canvas.present();

        Ok(Self {
            sdl_context,
            video_subsystem,
            canvas,
            event_pump,
            times_polled: 0,
        })
    }
}

impl Render {
    pub fn tick(&mut self) -> Result<(), RenderError> {
        self.times_polled += 1;
        // Put loop here
        // let i = (self.times_polled % 255) as u8;
        // self.canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        // self.canvas.clear();
        // let p1 = Point::new(40, 40);
        // let p2 = Point::new(40, 41);
        // let rec = Rect::new(50, 50, 100, 100);
        // self.canvas.set_draw_color(Color::RGB(255, 0, 0));
        //
        // self.canvas.fill_rect(Rect::new(0, 0, 100, 100))?;

        // let i: u8 = (self.times_polled as u8) % 255;
        // self.canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        // self.canvas.clear();
        // let rec = Rect::new(50, 50, 100, 100);

        let rerender = self.handle_events()?;
        // if self.handle_events()? {
        //     println!("Re-rendering");
        //     self.canvas.present();
        // };
        // self.canvas.present();
        Ok(())
    }

    pub fn handle_events(&mut self) -> Result<bool, RenderError> {
        let mut update_needed = false;
        while let Some(event) = self.event_pump.poll_event() {
            update_needed = self.handle_event(event)? || update_needed;
        }
        Ok(false)
    }

    fn handle_event(&mut self, event: Event) -> Result<bool, RenderError> {
        match event {
            Event::Quit { .. } => Err(RenderError::Quit),
            Event::MouseButtonDown { .. }
            | Event::MouseButtonUp { .. }
            | Event::MouseWheel { .. }
            | Event::MouseMotion { .. } => self.handle_mouse_event(event),
            _ => Ok(true),
        }
    }

    // TODO Proper structs and handling for these
    fn handle_mouse_event(&mut self, mouse_event: Event) -> Result<bool, RenderError> {
        match mouse_event {
            Event::MouseButtonDown { x, y, .. } => {
                println!("Mouse button down at ({}, {})", x, y);
                self.canvas.set_draw_color(Color::RGB(0, 255, 0));
                self.canvas.fill_rect(Rect::new(x, y, 10, 10))?;
                Ok(true)
            }
            _ => Ok(false)
        }
    }
}

/// Error type for the render module.
#[derive(Debug)]
pub enum RenderError {
    SdlError(String),
    WindowError(WindowBuildError),
    IntegerOrSdlError(IntegerOrSdlError),
    // InternalError(String),
    Quit,
}

impl From<String> for RenderError {
    fn from(err: String) -> Self {
        Self::SdlError(err)
    }
}

impl From<WindowBuildError> for RenderError {
    fn from(err: WindowBuildError) -> Self {
        Self::WindowError(err)
    }
}

impl From<IntegerOrSdlError> for RenderError {
    fn from(err: IntegerOrSdlError) -> Self {
        Self::IntegerOrSdlError(err)
    }
}
