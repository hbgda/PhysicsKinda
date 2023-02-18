pub mod core;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

fn main() {
    let mut renderer = core::renderer::Renderer::init();
   
    renderer.present();

    let mut event_pump = renderer.sdl_context.event_pump().unwrap();

    'event_loop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } 
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'event_loop,
                _ => {}
            } 
        } 
    }
}
