extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::TextureQuery;
use std::collections::HashSet;
use std::time::Duration;
use std::io::Result;

macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);

fn display_keys(keys: HashSet<sdl2::keyboard::Keycode>, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
    println!("Displaying keys {:?}", keys);
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.draw_rect(Rect::new(20, 20, 50, 50)).unwrap();
}

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo: Video", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let ttf_context = sdl2::ttf::init().unwrap();
    // Load a font
    let mut font = ttf_context.load_font("DejaVuSans-Bold.ttf", 72).unwrap();
    font.set_style(sdl2::ttf::STYLE_BOLD);

    let surface = font.render("H")
            .blended(Color::RGBA(0, 0, 0, 255)).unwrap();
    let texture = texture_creator.create_texture_from_surface(&surface).unwrap();
    let TextureQuery { width, height, .. } = texture.query();

    'running: loop {
        canvas.set_draw_color(Color::RGB(255, 0, 0));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                // Event::KeyDown {_, _, code, _, _, _} => display_key(code),
                _ => {}
            }
        }
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        let keys: HashSet<_> = event_pump.keyboard_state().pressed_scancodes().filter_map(Keycode::from_scancode).collect();
        if !keys.is_empty() {
            display_keys(keys, &mut canvas);
        }
        let target = rect!(150, 150, width, height);
        canvas.copy(&texture, None, Some(target)).unwrap();
        canvas.present();

    }
}