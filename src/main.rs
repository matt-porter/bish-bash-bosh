extern crate sdl2;
extern crate rand;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::TextureQuery;

use rand::{Rng, thread_rng};

use std::collections::HashSet;
use std::time::Duration;

macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);

/// Return a random position that fits rect within rect
fn random_position(rect: Rect, within_rect: Rect) -> Rect {
    let rx: f64 = thread_rng().gen();
    let ry: f64 = thread_rng().gen();
    let posx = rx * (within_rect.width() - 2 * rect.width()) as f64;
    let posy = ry * (within_rect.height() - 2 * rect.height()) as f64;
    rect!(posx + rect.width() as f64, posy + rect.height() as f64, rect.width(), rect.height())
}


fn display_keys(keys: HashSet<sdl2::keyboard::Keycode>, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
    println!("Displaying keys {:?}", keys);
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.draw_rect(Rect::new(20, 20, 50, 50)).unwrap();
}

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let (window_width, window_height) = (800, 600);
    let window = video_subsystem.window("rust-sdl2 demo: Video", window_width, window_height)
        .position_centered()
        .opengl()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let ttf_context = sdl2::ttf::init().unwrap();
    // Load a font
    let mut font = ttf_context.load_font("DejaVuSans-Bold.ttf", 64).unwrap();
    font.set_style(sdl2::ttf::STYLE_BOLD);
    
    canvas.set_draw_color(Color::RGB(255, 0, 0));
    canvas.clear();
    canvas.present();
    
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::Return), .. } => {
                    canvas.set_draw_color(Color::RGB(255, 0, 0));
                    canvas.clear();
                    canvas.present();
                },
                Event::KeyDown { keycode: Some(key), .. } 
                => {
                    let surface = font.render(&key.name())
                        .blended(Color::RGBA(0, 0, 0, 255)).unwrap();
                    let texture = texture_creator.create_texture_from_surface(&surface).unwrap();
                    let TextureQuery { width, height, .. } = texture.query();
                    let target = random_position(
                        rect!(0, 0, width, height),
                        rect!(0, 0, window_width, window_height)); //rect!(150, 150, width, height);
                    canvas.copy(&texture, None, Some(target)).unwrap();
                    canvas.present();
                },
                _ => {}
            }
        }
        // let keys: HashSet<_> = event_pump.keyboard_state().pressed_scancodes().filter_map(Keycode::from_scancode).collect();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));              
    }
}