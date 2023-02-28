use std::{collections::HashMap, rc::Rc, hash::Hash, borrow::Borrow, marker::PhantomData};

use sdl2::{render::{Texture, TextureCreator}, image::LoadTexture};

pub type TextureManager<'l, T> = AssetManager<'l, String, Texture<'l>, TextureCreator<T>>; 

pub struct AssetManager<'l, K, A, S>
where
    K: Eq + Hash,
    S: 'l + AssetSource<'l, A>
{
    source: Rc<S>,
    cache: HashMap<K, Rc<A>>,
    phantom: PhantomData<&'l A>
}

impl<'l, K, A, S> AssetManager<'l, K, A, S>
where
    K: Eq + Hash,
    S: 'l + AssetSource<'l, A>
{
    pub fn new(source: Rc<S>) -> Self {
        AssetManager { source, cache: HashMap::new(), phantom: PhantomData }
    }

    pub fn load<I>(&mut self, info: &I) -> Result<Rc<A>, String> 
    where
        S: AssetSource<'l, A, Args = I>,
        I: Eq + Hash + ?Sized,
        K: Borrow<I> + for<'a> From<&'a I>
    {
        self.cache.get(info).cloned().map_or_else(|| {
            let asset = Rc::new(self.source.load(info)?);
            self.cache.insert(info.into(), asset.clone());
            Ok(asset)
        }, Ok)
    }
}


pub trait AssetSource<'l, A> {
    type Args: ?Sized;
    fn load(&'l self, args: &Self::Args) -> Result<A, String>;
}

impl<'l, T> AssetSource<'l, Texture<'l>> for TextureCreator<T> {
    type Args = str;
    
    fn load(&'l self, path: &Self::Args) -> Result<Texture<'l>, String> {
        self.load_texture(path)
    }
}