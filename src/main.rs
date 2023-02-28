pub mod core;

use std::{thread, time::Duration};

use struct_extension::*;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use crate::core::physics::vector::Vector;
use crate::core::physics::{engine::Engine, entity::PhysicsEntity};
use crate::core::rendering::renderer::Renderer;

const FRAME_TIME: Duration = Duration::from_millis(1_000 / 60);

fn main() {
    
    let mut engine = Engine::new();

    let (id, e) = engine.entities.create_entity();
    e.position.set(0, 0);
    e.size.set(20, 20);
    e.velocity.set(0, 0);

    let (_, e2) = engine.entities.create_entity();
    e2.size.set(60, 40);

    let (sdl, mut renderer) = Renderer::init(850, 600, true);

    let mut event_pump = sdl.event_pump().unwrap();

    'event_loop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } 
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'event_loop,
                Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                    engine.entities.get_entity_mut(id).unwrap().velocity += Vector::<i32>::new(0, -15);
                }
                _ => {}
            } 
        } 

        engine.update();

        renderer.refresh(&engine.entities.all(), &event_pump);

        thread::sleep(FRAME_TIME);
    }
}
