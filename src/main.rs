extern crate sdl2;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
mod cpu;

fn main() -> Result<(), String> {
	/*
    let args: Vec<String> = env::args().collect();
    let path = Path::new(&args[1]);
    let display = path.display();
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {} : {}", display, why),
        Ok(file) => file,
    };
	let mut byte_buffer = Vec::new();
	file.read_to_end(&mut byte_buffer);
    let mut memory : [u8 ; 8000] = [0 ; 8000];
    for i in (0..byte_buffer.len()+1).step_by(2) {
        memory[512+i] = byte_buffer[i];
        memory[512+i+1] = byte_buffer[i+1];
    }
	*/



    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

	// NOTE: Dimension (160 x 144) multiplied by a factor of 5 for viewability
    let window = video_subsystem.window("Fierce Deity's GB", 800, 720)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window.into_canvas().build()
        .expect("could not make a canvas");

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump()?;
    let mut i = 0;
    'running: loop {
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
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

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    Ok(())
}




