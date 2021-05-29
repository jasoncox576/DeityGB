extern crate sdl2;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::time::Duration;

mod cpu;
mod mmu;

fn main() -> Result<(), String> {
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
    for i in (0..byte_buffer.len()).step_by(2) {
        instruction_buffer[i] = byte_buffer[i];
        instruction_buffer[i+1] = byte_buffer[i+1];
        println!("{:#04x}", instruction_buffer[i]);
        println!("{:#04x}", instruction_buffer[i+1]);
    }

    let mmu : mmu::MMU {
        memory: [0 ; 0x10000], 
    };

    let cpu : cpu::CPU {
        a : 0, f : 0,
        b : 0, c : 0,
        d : 0, e : 0,
        h : 0, l : 0,
        mmu_ref : &mut mmu,
        sp : 0,
        pc : 0,
    };


    // dimension: 160x144x3 (each pixel has R,G,B component)
    let screen_data : [u8 ; 69120] = [0 ; 69120];
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

	// NOTE: Dimension (160 x 144) multiplied by a factor of 5 for viewability
    let window = video_subsystem.window("Fierce Deity's GB", 800, 720)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window.into_canvas().build()
        .expect("could not make a canvas");

    let texture_creator : TextureCreator = canvas.texture_creator();
    let pixel_format : sdl2::pixels::PixelFormatEnum = sdl2::pixels::PixelFormatEnum::RGB24;
    let texture : Texture = texture_creator.create_texture_streaming<pixel_format>(
    format: pixel_format,
    width: 160,
    height: 144,
    ).unwrap();

    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump()?;
    let mut i = 0;
    'running: loop {
        i = (i + 1) % 255;
        texture.update<None>(
            rect: None, 
            pixel_data: &screen_data[..],
            pitch: 160,
        );
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        let opcode = instruction_buffer[cpu.pc as usize] 
        cpu.decode_execute(opcode);

        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    Ok(())
}






