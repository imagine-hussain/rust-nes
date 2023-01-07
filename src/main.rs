use std::time::Duration;

use lib::rendering::Render;

pub fn main() {

    // let sdl_context = sdl2::init().unwrap();
    // let video_subsystem = sdl_context.video().unwrap();
    // let window = video_subsystem
    //         .window("rust nes", 800, 600)
    //         .position_centered()
    //         .build()
    //         .unwrap();
    //
    // let mut canvas = window.into_canvas().build().unwrap();
    //
    // canvas.set_draw_color(Color::RGB(0, 255, 255));
    // canvas.clear();
    // canvas.present();
    // let mut event_pump = sdl_context.event_pump().unwrap();
    // let mut i = 0;
    let mut r = Render::init().unwrap();
    'running: loop {
        if r.tick().is_err() {
            break 'running;
        }

        // Context::new();
        // i = (i + 1) % 255; canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        // canvas.clear();
        // let p1 = Point::new(40, 40);
        // let p2 = Point::new(40, 41);
        // let rec = Rect::new(50, 50, 100, 100);
        // canvas.set_draw_color(Color::RGB(255, 0, 0));
        //
        // for event in event_pump.poll_iter() {
        //     match event {
        //         Event::Quit { .. }
        //         | Event::KeyDown {
        //             keycode: Some(Keycode::Escape),
        //             ..
        //         } => break 'running,
        //         _ => {}
        //     }
        // }
        //
        // canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

}
