extern crate hsl;
extern crate sdl2;
extern crate rand;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::TextureQuery;
use sdl2::video::FullscreenType;
use sdl2::audio::{AudioCallback, AudioSpecDesired,AudioSpecWAV,AudioCVT};

use hsl::HSL;
use rand::{Rng, thread_rng};
use std::collections::HashSet;

use std::time::Duration;
use std::borrow::Cow;
use std::path::{PathBuf, Path};

macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);

struct Sound {
    data: Vec<u8>,
    volume: f32,
    pos: usize,
}

impl AudioCallback for Sound {
    type Channel = u8;

    fn callback(&mut self, out: &mut [u8]) {
        for dst in out.iter_mut() {
            *dst = (*self.data.get(self.pos).unwrap_or(&0) as f32 * self.volume) as u8;
            self.pos += 1;
        }
    }
}

/// Return a random position that fits rect within rect
fn random_position(rect: Rect, within_rect: Rect) -> Rect {
    let rx: f64 = thread_rng().gen();
    let ry: f64 = thread_rng().gen();
    let posx = rx * (within_rect.width() - 1 * rect.width()) as f64;
    let posy = ry * (within_rect.height() - 1 * rect.height()) as f64;
    rect!(posx as f64, posy  as f64, rect.width(), rect.height())
}

fn random_colour() -> Color {
    let h: f64 = thread_rng().gen();
    let blue = HSL { h: h * 360.0, s: 1_f64, l: 0.5_f64 };
    let rgb = blue.to_rgb();
    return Color::RGB(rgb.0, rgb.1, rgb.2);
}


fn display_keys(keys: HashSet<sdl2::keyboard::Keycode>, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
    println!("Displaying keys {:?}", keys);
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.draw_rect(Rect::new(20, 20, 50, 50)).unwrap();
}

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let audio_subsystem = sdl_context.audio().unwrap();
    let (window_width, window_height) = (800, 600);
    let mut window = video_subsystem.window("Bish Bash Bosh", window_width, window_height)
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
    // Load a sound
    let wav_file : Cow<'static, Path> = Cow::from(Path::new("./sounds/68437__pinkyfinger__piano-a.wav"));
    let desired_spec = AudioSpecDesired {
            freq: Some(44_100),
            channels: Some(1), // mono
            samples: None      // default
    };
    
    canvas.set_draw_color(Color::RGB(255, 0, 0));
    canvas.clear();
    canvas.present();

    ///
    /// Keep track of all displayed characters, and their postitions
    /// 

    let mut drawables = vec![];
    let drawable_keys : HashSet<String> = ["A", "B", "C", "D", "E", "F", "G", "H", "I", "J",
                                         "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T",
                                         "U", "V", "W", "X", "Y", "Z", "0", "1", "2", "3",
                                         "4", "5", "6", "7", "8", "9", ].iter().map(|s| s.to_string()).collect();
    let mut background_color = random_colour();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::Return), .. } => {
                    drawables.clear();
                    background_color = random_colour();
                },
                Event::KeyDown { keycode: Some(Keycode::F1), .. } => {
                    // Need to dump noise in a background thread or something
                    let device = audio_subsystem.open_playback(None, &desired_spec, |spec| {
                        let wav = AudioSpecWAV::load_wav(wav_file.clone())
                            .expect("Could not load test WAV file");

                        let cvt = AudioCVT::new(
                                wav.format, wav.channels, wav.freq,
                                spec.format, spec.channels, spec.freq)
                            .expect("Could not convert WAV file");

                        let data = cvt.convert(wav.buffer().to_vec());

                        // initialize the audio callback
                        Sound {
                            data: data,
                            volume: 0.25,
                            pos: 0,
                        }
                    }).unwrap();

                    // Start playback
                    device.resume();
                    // Play for a second
                    std::thread::sleep(Duration::from_millis(1_000));
                },
                Event::KeyDown { keycode: Some(key), .. } 
                => {
                    if drawable_keys.contains(&key.name()) {
                        let colour = random_colour();
                        let surface = font.render(&key.name())
                            .blended(colour).unwrap();
                        let texture = texture_creator.create_texture_from_surface(&surface).unwrap();
                        let TextureQuery { width, height, .. } = texture.query();
                        let target = random_position(
                            rect!(0, 0, width, height),
                            rect!(0, 0, window_width, window_height)); //rect!(150, 150, width, height);
                        drawables.push((texture, target));
                    }
                },
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
        // let keys: HashSet<_> = event_pump.keyboard_state().pressed_scancodes().filter_map(Keycode::from_scancode).collect();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));              
    }
} 
