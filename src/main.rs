use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::collections::HashMap;
use std::collections::HashSet;
use std::time::{Duration, Instant};
use ggez::*;
use ggez::event::{KeyCode, KeyMods};
use ggez::input::keyboard;
use ggez::audio::SoundSource;
use ggez::nalgebra as na;
use std::thread;


mod cpu;
mod mmu;

struct State {
    dt: std::time::Duration,
    pc : usize,
    cpu : cpu::CPU,
    mmu : mmu::MMU,
    current_instruction: u16,
    delay : u64,
}

impl ggez::event::EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.dt = timer::delta(ctx);
        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, _keycode: KeyCode, _keymods: KeyMods, _repeat: bool) {
        if _keycode == KeyCode::Escape {
            event::quit(ctx);
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
		let mut screen_bitmap_rgba : [u8 ; 92160] = [0 ; 92160];

		for i in 0..23040 {
			let j = (4*i) as usize;
			let set_val = 255;
			//screen_bitmap_rgba[j] = set_val;
			screen_bitmap_rgba[j+1] = set_val;
			//screen_bitmap_rgba[j+2] = set_val;
			// alpha channel
			screen_bitmap_rgba[j+3] = 255;
		}
		let screen_slice = &screen_bitmap_rgba[..];
		let screen : graphics::Image = ggez::graphics::Image::from_rgba8(ctx, 160, 144, screen_slice).unwrap(); 
		graphics::draw(ctx, &screen, (na::Point2::new(0.0,0.0),))?;

		// actually puts stuff on the screen
		graphics::present(ctx)?;
        Ok(())
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let path = Path::new(&args[1]);
    let display = path.display();
    let mut cartridge = match File::open(&path) {
        Err(why) => panic!("couldn't open {} : {}", display, why),
        Ok(cartridge) => cartridge,
    };
    let mut byte_buffer = Vec::new();
    cartridge.read_to_end(&mut byte_buffer);
    let mut instruction_buffer : [u8 ; 8000] = [0 ; 8000];
    for i in (0..byte_buffer.len()-1).step_by(2) {
        instruction_buffer[i] = byte_buffer[i];
        instruction_buffer[i+1] = byte_buffer[i+1];
        println!("{:#04x}", instruction_buffer[i]);
	}
	let mmu = mmu::MMU::new();
    let cpu = cpu::CPU::new();

    let state = &mut State { dt: std::time::Duration::new(0, 0), pc : 0, cpu : cpu, mmu : mmu, current_instruction : 0, delay: 1};

    let window_cfg : conf::WindowMode = conf::WindowMode {
        width : 800.0,
        height : 720.0,
        maximized: false,
        fullscreen_type: conf::FullscreenType::Windowed,
        borderless: false,
        min_width: 0.0,
        max_width: 0.0,
        min_height: 0.0,
        max_height: 0.0,
        resizable: false,
    };

    let c = conf::Conf::new();
    let (ref mut ctx, ref mut event_loop) = ContextBuilder::new("DeityGB", "jasoncox")
        .conf(c)
        .build()
        .unwrap();

    graphics::set_default_filter(ctx, graphics::FilterMode::Nearest);
    let rect : graphics::Rect = graphics::Rect::new(0.0, 0.0, 160.0, 144.0);

    graphics::set_mode(ctx, window_cfg);
    graphics::set_screen_coordinates(ctx, rect);
    graphics::set_window_title(ctx, "Deity's GB");

    // run main loop
    event::run(ctx, event_loop, state).unwrap();
}
