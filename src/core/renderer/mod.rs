pub mod ext;

use sdl2::{self, pixels::Color, rect::Rect};

use super::physics::entity::PhysicsEntity;

pub struct Renderer {
    pub sdl_context: sdl2::Sdl,
    pub canvas: sdl2::render::WindowCanvas
}

impl Renderer {
    pub fn init() -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video = sdl_context.video().unwrap();

        let window = video.window("Tings innit", 800, 600)
            .position_centered()
            .opengl()
            .build()
            .map_err(|e| e.to_string()).unwrap();

        let canvas = window.into_canvas()
            .build()
            .map_err(|e| e.to_string()).unwrap();

        Renderer {
            sdl_context, canvas
        }
    }

    pub fn clear(&mut self) {
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();
    }

    pub fn present(&mut self) {
        self.canvas.present();
    }

    pub fn update(&mut self, entities: &Vec<&PhysicsEntity>) {
        self.canvas.set_draw_color(Color::WHITE);

        for entity in entities {
            self.draw_entity(entity);
        }
    }

    pub fn draw_entity(&mut self, entity: &PhysicsEntity) {
        let rect: Rect = entity.into();
        let _ = self.canvas.fill_rect(rect);        
    }
}
