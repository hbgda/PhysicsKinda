pub mod core;
pub mod line_test;

use std::time::Instant;
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

    // return line_test::line_test();

    let viewport = Vector::<u32>::new(850, 600);
    let mut engine = Engine::new(viewport);

    let (id, e) = engine.entities.create_entity();
    e.position.set(0, -100);
    e.size.set(20, 20);
    e.velocity.set(0, 0);

    /*
    let (_, e2) = engine.entities.create_entity();
    e2.position.set(0, 200);
    e2.size.set(850, 10);
    e2.material.gravity = false;
    */
    let (sdl, mut renderer) = Renderer::init(viewport.x(), viewport.y(), true);

    let mut event_pump = sdl.event_pump().unwrap();

    'event_loop: loop {
        let start = Instant::now();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } 
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'event_loop,
                Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                    engine.entities.get_entity_mut(id).unwrap().velocity += Vector::<i32>::new(5, 0);
                },
                Event::MouseButtonDown { mouse_btn: MouseButton::Left, x, y, .. } => {
                    let (id, new_entity) = engine.entities.create_entity();
                    new_entity.position.set(
                        x - (renderer.engine_viewport.x() / 2) as i32,
                        y - (renderer.engine_viewport.y() / 2) as i32 
                    );
                    new_entity.size.set(20, 20);
                    println!("Created new entity: {:?} at X: {x}, Y: {y}", id);
                }
                _ => {}
            } 
        }
        // println!("{:?}", engine.entities.get_entity(id).unwrap().velocity);

        engine.update();
        renderer.refresh(&engine.entities.all(), &event_pump);

        let elapsed = start.elapsed();
        if let Some(frame_delay) = FRAME_TIME.checked_sub(elapsed) {    
            thread::sleep(frame_delay);
        }
    }
}
