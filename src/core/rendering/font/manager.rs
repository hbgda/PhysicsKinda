use sdl2::ttf::{Sdl2TtfContext, Font};
use std::{collections::HashMap, rc::Rc};

pub struct FontManager {
    ttf: Sdl2TtfContext,
    // cache: HashMap<FontInfo, Font<'f, 'static>>
}

impl FontManager {
    pub fn new(ttf_context: Sdl2TtfContext) -> Self {
        FontManager {
            ttf: ttf_context,
            // cache: HashMap::new()
        }
    }
}

impl FontManager {
    pub fn load(&mut self, info: &FontInfo) -> Option<Font> {
        if let Ok(font) = self.ttf.load_font(info.path, info.size) {
            return Some(font);
        }
        None
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct FontInfo {
    pub path: &'static str,
    pub size: u16 
}