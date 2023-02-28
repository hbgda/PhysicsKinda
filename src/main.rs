pub mod core;

use std::{thread, time::Duration};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;

use crate::core::physics::units::TPS;
use crate::core::physics::vector::Vector;
use crate::core::physics::engine::Engine;
use crate::core::rendering::renderer::Renderer;

const FRAME_TIME: Duration = Duration::from_millis(1_000 / TPS as u64);

fn main() {
    
    let mut engine = Engine::new();

    let (id, e) = engine.entities.create_entity();
    e.position.set(0, -100);
    e.size.set(20, 20);
    e.velocity.set(0, 0);

    let (_, floor) = engine.entities.create_entity();
    floor.size.set(800, 50);
    floor.position.set(0, 230);
    floor.properties.gravity = false;

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
                    engine.entities.get_entity_mut(id).unwrap().position += Vector::<i32>::new(0, 15);
                },
                Event::MouseButtonDown { mouse_btn: MouseButton::Left, x, y, .. } => {
                    let (id, new_entity) = engine.entities.create_entity();
                    new_entity.position.set(
                        x - (renderer.viewport.x() / 2) as i32,
                        y - (renderer.viewport.y() / 2) as i32 
                    );
                    new_entity.size.set(20, 20);
                    println!("Created new entity: {:?} at X: {x}, Y: {y}", id);
                }
                _ => {}
            } 
        } 

        engine.update();

        renderer.refresh(&engine.entities.all(), &event_pump);

        thread::sleep(FRAME_TIME);
    }
}
