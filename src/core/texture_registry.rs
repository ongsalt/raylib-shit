use std::{collections::HashMap, rc::Rc};

use raylib::prelude::*;

pub struct TextureRegistry {
    registry: HashMap<String, Rc<Texture2D>>,
}

impl TextureRegistry {
    pub fn new() -> Self {
        Self {
            registry: HashMap::new(),
        }
    }

    pub fn load(
        &mut self,
        id: &str,
        filename: &str,
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
    ) -> Rc<Texture2D> {
        let image =
            Image::load_image(&filename).expect(format!("File not found: {}", filename).as_str());
        let texture = rl
            .load_texture_from_image(thread, &image)
            .expect(format!("Unable to load texture from {}", filename).as_str());
        self.add(id, texture)
    }

    pub fn load_if_not_existed(
        &mut self,
        id: &str,
        filename: &str,
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
    ) -> Rc<Texture2D> {
        if let Some(texture) = self.get(id) {
            return texture;
        }
        self.load(id, filename, rl, thread)
    }

    pub fn add(&mut self, id: &str, texture: Texture2D) -> Rc<Texture2D> {
        let t = Rc::new(texture);
        self.registry.insert(id.into(), t.clone());
        t
    }

    pub fn remove(&mut self, id: &str) -> Option<Rc<Texture2D>> {
        self.registry.remove(id)
    }

    pub fn has(&self, id: &str) -> bool {
        self.registry.contains_key(id)
    }

    pub fn get(&self, id: &str) -> Option<Rc<Texture2D>> {
        self.registry.get(id).cloned()
    }
}
