pub mod core;

use std::thread;

use struct_extension::*;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use crate::core::physics::{engine::Engine, entity::PhysicsEntity};

fn main() {
    
    let mut engine = Engine::new();

    let (_, e) = engine.entities.create_entity();
    e.position.set(10, 10);
    e.size.set(20, 20);
    e.velocity.set(1, 0);

    let mut renderer = core::renderer::Renderer::init();

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

        engine.update();

        renderer.clear();
        renderer.update(&engine.entities.all());
        renderer.present();

        // thread::sleep()
    }
}
