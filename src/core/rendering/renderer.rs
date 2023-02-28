use std::rc::Rc;

use sdl2::event::WindowEvent;
use sdl2::render::{WindowCanvas, TextureCreator, TextureQuery};
use sdl2::video::WindowContext;
use sdl2::{self, pixels::Color, rect::Rect};
use sdl2::messagebox::*;

// use super::texture::manager::TextureManager;
use super::{ext::entity::PhysicsEntityExt, font::{manager::FontManager, presets}};

use crate::core::physics::{entity::PhysicsEntity, vector::Vector};

pub struct Renderer {
    pub viewport: Vector<u32>,
    // pub sdl_context: sdl2::Sdl,
    pub canvas: sdl2::render::WindowCanvas,
    font_manager: FontManager,
    texture_creator: Rc<TextureCreator<WindowContext>>,
    // texture_manager: TextureManager<'l>,
    pub debug: bool
}

impl Renderer {
    pub fn init(width: u32, height: u32, debug: bool) -> (sdl2::Sdl, Self) {
        let sdl_context = sdl2::init().unwrap();        
        let video = sdl_context.video().unwrap();

        let ttf_context = sdl2::ttf::init().unwrap();
        // let mut font = ttf_context.load_font("font/roboto/Roboto-Regular.ttf", 128).unwrap();

        let window = video.window("Tings innit", width, height)
            .position_centered()
            .opengl()
            .build()
            .map_err(|e| e.to_string()).unwrap();

        let canvas = window.into_canvas()
            .build()
            .map_err(|e| e.to_string()).unwrap();

        let texture_creator = canvas.texture_creator();
        // let texture_manager = TextureManager::new(Rc::new(texture_creator));

        (sdl_context, Renderer {
            canvas, viewport: Vector::<u32>::new(width, height), font_manager: FontManager::new(ttf_context), texture_creator: Rc::new(texture_creator), debug
        })
    }

    pub fn refresh(&mut self, entities: &Vec<&PhysicsEntity>, event_pump: &sdl2::EventPump) {
        self.clear();
        self.update(entities, event_pump);
        self.present();
    }

    pub fn clear(&mut self) {
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();
    }

    pub fn present(&mut self) {
        self.canvas.present();
    }

    pub fn update(&mut self, entities: &Vec<&PhysicsEntity>, event_pump: &sdl2::EventPump) {
        if self.debug {
            self.draw_debug(event_pump);
        }

        for entity in entities {
            self.draw_entity(entity);
        }
    }

    pub fn draw_entity(&mut self, entity: &PhysicsEntity) {
        self.canvas.set_draw_color(Color::WHITE);
        let rect: Rect = entity.to_rect(self.viewport);
        let _ = self.canvas.fill_rect(rect);        
    }
}

impl Renderer {
    pub fn draw_debug(&mut self, event_pump: &sdl2::EventPump) {
        // Draw grid
        self.debug_draw_grid();

        // Draw mouse pos
        self.debug_draw_mouse_pos(event_pump);
    }

    fn debug_draw_grid(&mut self) {
        self.canvas.set_draw_color(Color::GREY);
        
        let cell_size = 50;
        let x_cells = self.viewport.x() / cell_size;
        let x_cell_offset = self.viewport.x() - (x_cells * cell_size);
        let x_offset = cell_size - x_cell_offset / 2;  

        let y_cells = self.viewport.y() / cell_size;
        let y_cell_offset = self.viewport.y() - (y_cells * cell_size);
        let y_offset = cell_size - y_cell_offset / 2;

        let mut y = 0;   
        while y < self.viewport.y() + y_offset {
            let mut x = 0;
            while x < self.viewport.x() + x_offset {
                let rect = Rect::new(x as i32 - x_offset as i32, y as i32 - y_offset as i32, cell_size, cell_size);
                let _ = self.canvas.draw_rect(rect);
                x += cell_size;
            }
            y += cell_size;
        }
    }

    fn debug_draw_mouse_pos(&mut self, event_pump: &sdl2::EventPump) {
        self.canvas.set_draw_color(Color::WHITE);
        let font = self.font_manager.load(&presets::ROBOTO);
        if let Some(font) = font {
            let mouse_state = event_pump.mouse_state();
            let mouse_x = mouse_state.x();
            let mouse_y = mouse_state.y();

            let text_surface = font
                .render(&format!("X: {mouse_x}, Y: {mouse_y}"))
                .blended(Color::WHITE)
                .map_err(|e| e.to_string()).unwrap();

            let texture = self.texture_creator.create_texture_from_surface(text_surface).unwrap();
            let TextureQuery { width, height, .. } = texture.query();

            let width = width / 5;
            let height = height / 5;
            let rect = Rect::new(
                (self.viewport.x() - width) as i32,
                (self.viewport.y() - height) as i32,
                width, height
            );

            let _ = self.canvas.copy(&texture, None, rect);
        }
        else {
            let _ = show_message_box(
                MessageBoxFlag::ERROR,
                vec![
                    ButtonData {
                        flags: MessageBoxButtonFlag::ESCAPEKEY_DEFAULT,
                        button_id: 1,
                        text: "Ok"
                    }
                ].as_slice(),
                "Missing Font File", 
                "Default font files cannot be found, make sure you have extracted them to the project folder.", 
                self.canvas.window(),
                None
            );
            panic!("Missing font files");
        }
    }
}