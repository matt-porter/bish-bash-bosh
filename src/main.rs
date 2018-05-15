extern crate hsl;
extern crate rand;
extern crate sdl2;

use sdl2::audio::{AudioCVT, AudioSpecDesired, AudioSpecWAV, AudioQueue};
use sdl2::event::Event;
use sdl2::image::{INIT_PNG, LoadSurface};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{TextureQuery};
use sdl2::surface::Surface;
use sdl2::video::FullscreenType;

use hsl::HSL;
use rand::{thread_rng, Rng};
use std::collections::{HashSet, HashMap};

use std::borrow::Cow;
use std::path::{Path, PathBuf};
use std::time::Duration;

macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);

trait PositionStrategy {
    fn next_position(&mut self, rect: Rect, within_rect: Rect) -> Rect;
    fn reset(&mut self) {

    }
}

struct RandomPositionStrategy {}

impl PositionStrategy for RandomPositionStrategy {
    // Return a random position that fits rect within rect
    fn next_position(&mut self, rect: Rect, within_rect: Rect) -> Rect {
        let rx: f64 = thread_rng().gen();
        let ry: f64 = thread_rng().gen();
        let posx = rx * (within_rect.width() - 1 * rect.width()) as f64;
        let posy = ry * (within_rect.height() - 1 * rect.height()) as f64;
        rect!(posx as f64, posy as f64, rect.width(), rect.height())
    }
}

struct SequentialPositionStrategy {
    next_x: u32
}

impl PositionStrategy for SequentialPositionStrategy {    
    fn next_position(&mut self, rect: Rect, within_rect: Rect) -> Rect {
        let y = within_rect.height() / 2;
        let x = self.next_x;
        self.next_x = x + rect.width();
        rect!(x, y, rect.width(), rect.height())
    }

    fn reset(&mut self) {
        self.next_x = 0;
    }
}

fn random_colour(c: Color) -> Color {
    let not_near_hsl = HSL::from_rgb(&[c.r, c.g, c.b]);
    let mut generated = not_near_hsl.clone();
    while (generated.h - not_near_hsl.h).abs() < 40. {
        let h: f64 = thread_rng().gen();    
        generated = HSL {
            h: h * 360.0,
            s: 1_f64,
            l: 0.5_f64,
        };
    }
    let rgb = generated.to_rgb();
    return Color::RGB(rgb.0, rgb.1, rgb.2);
}

fn load_sound(note: &str) -> AudioSpecWAV {
    // Load a sound
    let filename = format!("{}.wav", note);
    let path: PathBuf = ["./sounds", &filename].iter().collect();
    let wav_file: Cow<'static, Path> = Cow::from(path);
    AudioSpecWAV::load_wav(wav_file.clone())
        .expect("Could not load test WAV file")
}

fn load_image(fname: &str) -> Surface {
    // Load a sound
    let filename = format!("{}.png", fname);
    let path: PathBuf = ["./images", &filename].iter().collect();
    let image_file: Cow<'static, Path> = Cow::from(path);
    Surface::from_file(image_file.clone())
        .expect("Could not load image file")
}

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let audio_subsystem = sdl_context.audio().unwrap();
    let image_context = sdl2::image::init(INIT_PNG);
    let (window_width, window_height) = (800, 600);
    let mut window = video_subsystem
        .window("Bish Bash Bosh", window_width, window_height)
        .position_centered()
        .opengl()
        .build()
        .unwrap();
    window.set_fullscreen(FullscreenType::Desktop).unwrap();
    window.set_grab(true);
    let (window_width, window_height) = window.size();
    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let ttf_context = sdl2::ttf::init().unwrap();
    // Load a font
    let mut font = ttf_context.load_font("DejaVuSans-Bold.ttf", 112).unwrap();
    font.set_style(sdl2::ttf::STYLE_BOLD);
    
    let desired_spec = AudioSpecDesired {
        freq: Some(44_100),
        channels: Some(1), // mono
        samples: None,     // default
    };

    let audio_queue: AudioQueue<u8> = audio_subsystem
        .open_queue(None, &desired_spec)
        .unwrap();

    // let mut position_strategy = RandomPositionStrategy { };
    let mut position_strategy = SequentialPositionStrategy { next_x: 0 };

    canvas.set_draw_color(Color::RGB(255, 0, 0));
    canvas.clear();
    canvas.present();

    // Keep track of all displayed characters, and their postitions
    let mut drawables = vec![];
    let drawable_keys: HashSet<String> = [
        "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R",
        "S", "T", "U", "V", "W", "X", "Y", "Z", "0", "1", "2", "3", "4", "5", "6", "7", "8", "9",
    ].iter()
        .map(|s| s.to_string())
        .collect();
    let noisy_keys: HashMap<String, String> = [
        ("F1", "37a"),
        ("F2", "38b"),
        ("F3", "39bb"),
        ("F4", "40c"),
        ("F5", "41c"),
        ("F6", "42d"),
        ("F7", "43e"),
        ("F8", "44eb"),
        ("F9", "45f"),
        ("F10", "46f"),
        ("F11", "47g"),
        ("F12", "48g"),
        ("A", "alpha-a"),
        ("B", "alpha-b"),
        ("C", "alpha-c"),
        ("D", "alpha-d"),
        ("E", "alpha-e"),
        ("F", "alpha-f"),
        ("G", "alpha-g"),
        ("H", "alpha-h"),
        ("I", "alpha-i"),
        ("J", "alpha-j"),
        ("K", "alpha-k"),
        ("L", "alpha-l"),
        ("M", "alpha-m"),
        ("N", "alpha-n"),
        ("O", "alpha-o"),
        ("P", "alpha-p"),
        ("Q", "alpha-q"),
        ("R", "alpha-r"),
        ("S", "alpha-s"),
        ("T", "alpha-t"),
        ("U", "alpha-u"),
        ("V", "alpha-v"),
        ("W", "alpha-w"),
        ("X", "alpha-x"),
        ("Y", "alpha-y"),
        ("Z", "alpha-z"),
        ].iter()
         .map(|(s1, s2)| (s1.to_string(), s2.to_string()))
         .collect();
    let images: HashMap<String, String> = [
        ("T", "T"),
        ].iter()
         .map(|(s1, s2)| (s1.to_string(), s2.to_string()))
         .collect();
    let mut background_color = random_colour(Color::RGB(255, 255, 255));
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    repeat: true,
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Return),
                    repeat: false,
                    ..
                } => {
                    position_strategy.reset();
                    drawables.clear();
                    background_color = random_colour(Color::RGB(255, 255, 255));
                }
                Event::KeyDown {
                    keycode: Some(key),
                    repeat: false, 
                    ..
                } => {
                    if drawable_keys.contains(&key.name()) {
                        let colour = random_colour(background_color);
                        let surface = font.render(&key.name()).blended(colour).unwrap();
                        let texture = texture_creator
                            .create_texture_from_surface(&surface)
                            .unwrap();
                        let TextureQuery { width, height, .. } = texture.query();
                        let target = position_strategy.next_position(
                            rect!(0, 0, width, height),
                            rect!(0, 0, window_width, window_height),
                        ); //rect!(150, 150, width, height);
                        drawables.push((texture, target));
                    }
                    if let Some(note) = noisy_keys.get(&key.name()) {
                        let wav = load_sound(&note);
                        let spec = audio_queue.spec();
                        let cvt = AudioCVT::new(
                            wav.format,
                            wav.channels,
                            wav.freq,
                            spec.format,
                            spec.channels,
                            spec.freq,
                        ).expect("Could not convert WAV file");

                        let data = cvt.convert(wav.buffer().to_vec());
                        audio_queue.clear();
                        audio_queue.queue(&data);
                        // Start playback
                        audio_queue.resume();
                    }
                    if let Some(filename) = images.get(&key.name()) {
                        let surface = load_image(&filename);
                        let texture = texture_creator
                            .create_texture_from_surface(&surface)
                            .unwrap();
                        let TextureQuery { width, height, .. } = texture.query();
                        let target = position_strategy.next_position(
                            rect!(0, 0, width, height),
                            rect!(0, 0, window_width, window_height),
                        );
                        drawables.push((texture, target));
                    }
                }
                _ => {}
            }
        }
        // Draw the chars
        canvas.set_draw_color(background_color);
        canvas.clear();
        for &(ref texture, target) in drawables.iter() {
            canvas.copy(&texture, None, Some(target.clone())).unwrap();
        }
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
