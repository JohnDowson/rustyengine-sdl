extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::ttf;
use std::path::Path;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let sdl_ttf_context = ttf::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let font = sdl_ttf_context
        .load_font(Path::new("./resources/OpenSans-Regular.ttf"), 48)
        .unwrap();
    let text = font
        .render("Hello world")
        .blended(Color::RGB(255, 255, 255))
        .unwrap();
    let window = video_subsystem
        .window("sdl2", 800, 600)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    let target = Rect::new(0, 0, 200, 100);
    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.create_texture_from_surface(&text).unwrap();
    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    'running: loop {
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        // The rest of the game loop goes here...
        canvas.copy(&texture, None, target).unwrap();
        canvas.present();
        //::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
