pub mod core;

use struct_extension::*;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use crate::core::physics::{engine::Engine, entity::PhysicsEntity};

#[extend(PhysicsEntity)]
struct Player {}

fn main() {
    let mut p = Player {
        position: (0.0, 0.0),
        velocity: (0.0, 0.0),
    };
    
    let mut engine = Engine::new();
    engine.register_entity::<Player>(&mut p);
    // let mut renderer = core::renderer::Renderer::init();
   
    // renderer.present();

    // let mut event_pump = renderer.sdl_context.event_pump().unwrap();

    // 'event_loop: loop {
    //     for event in event_pump.poll_iter() {
    //         match event {
    //             Event::Quit { .. } 
    //             | Event::KeyDown {
    //                 keycode: Some(Keycode::Escape),
    //                 ..
    //             } => break 'event_loop,
    //             _ => {}
    //         } 
    //     } 
    // }
}
