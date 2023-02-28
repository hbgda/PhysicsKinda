use std::{collections::HashMap, rc::Rc, path::PathBuf};

use sdl2::{render::{TextureCreator, Texture}, video::WindowContext, image::LoadTexture};

pub struct TextureManager<'l> {
    creator: Rc<TextureCreator<WindowContext>>,
    cache: HashMap<String, Rc<Texture<'l>>>
}

impl<'l> TextureManager<'l> {
    pub fn new(creator: Rc<TextureCreator<WindowContext>>) -> Self {
        TextureManager { creator, cache: HashMap::new() }
    }

    pub fn load(&mut self, id: String, path: &'static str) -> Option<Rc<Texture>> {
        if let Some(texture) = self.cache.get(id.as_str()) {
            return Some(texture.clone());
        }
        if let Ok(texture) = self.creator.load_texture(path) {
            let rc: Rc<Texture> = Rc::new(texture);
            self.cache.insert(String::from("ligma"), Rc::new(texture));
            return Some(rc);
        }
        None
    }
}