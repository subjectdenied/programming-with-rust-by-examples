extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::{TextureCreator};
use sdl2::image::{LoadTexture, INIT_PNG, INIT_JPG};
use std::time::Duration;

fn main() {
    let sdl_context = sdl2::init().expect("SDL initialization failed");
    let video_subsystem = sdl_context.video().expect("Couldn't get SDL video subsystem");

    sdl2::image::init(INIT_PNG | INIT_JPG).expect("Couldn't initialize image context");


    // Parameters are: title, width, height
    let window = video_subsystem.window("Tetris", 800, 600)
        .position_centered() // to put it in the middle of the screen
        .opengl()
        .build() // to create the window
        .expect("Failed to create window");

    let mut canvas = window.into_canvas()
        .target_texture()
        .present_vsync() // To enable v-sync.
        .build()
        .expect("Couldn't get window's canvas");

    let texture_creator: TextureCreator<_> = canvas.texture_creator();

    let image_texture = texture_creator.load_texture("assets/image.png")
        .expect("Couldn't load image");

    // First we get the event handler:
    let mut event_pump = sdl_context.event_pump().expect("Failed to get SDL event pump");

    // Then we create an infinite loop to loop over events:
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                // If we receive a 'quit' event or if the user press the 'ESC' key, we quit.
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running // We "break" the infinite loop.
                },
                _ => {}
            }
        }

        // We fill our window with red.
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        // We draw it.
        canvas.clear();

        canvas.copy(&image_texture, None, None).expect("Render failed");
        canvas.present();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
   }
}
