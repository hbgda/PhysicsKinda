
use std::time::Instant;
use std::{thread, time::Duration};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::rect::Rect;

use crate::core::physics::collision::line::{Line, Point, line_intersect};
use crate::core::physics::units::TPS;
use crate::core::physics::vector::Vector;
use crate::core::rendering::renderer::Renderer;

const FRAME_TIME: Duration = Duration::from_millis(1_000 / TPS as u64);

pub fn line_test() {
    let viewport = Vector::<u32>::new(300, 300);

    let (sdl, mut renderer) = Renderer::init(viewport.x(), viewport.y(), false);


    let mut line1 = Line(
        Point { x: 100, y: 10 },
        Point { x: 100, y: 100}
    );
    let mut line2 = Line(
        Point { x: 0, y: 50 },
        Point { x: 200, y: 40}
    );

    let mut event_pump = sdl.event_pump().unwrap();
    'event_loop: loop {
        let start = Instant::now();
        line1.0.x += 1;
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
        // println!("{:?}", engine.entities.get_entity(id).unwrap().velocity);

        renderer.clear();

        renderer.draw_line(
            (line1.0.x, line1.0.y), 
            (line1.1.x, line1.1.y)
        );
        renderer.draw_line(
            (line2.0.x, line2.0.y), 
            (line2.1.x, line2.1.y)
        );
        if let Some((intersect_x, intersect_y)) = line_intersect(line1, line2) {
            let _ = renderer.canvas.draw_rect(Rect::new(intersect_x - 5, intersect_y - 5, 10, 10));
        }

        renderer.present();

        let elapsed = start.elapsed();
        if let Some(frame_delay) = FRAME_TIME.checked_sub(elapsed) {    
            thread::sleep(frame_delay);
        }
    }
}
