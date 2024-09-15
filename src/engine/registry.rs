use std::{collections::HashMap, rc::Rc};

use raylib::texture::Texture2D;

pub struct TextureRegistry {
    registry: HashMap<String ,Rc<Texture2D>>
}

impl TextureRegistry {
    pub fn new() -> Self {
        Self {
            registry: HashMap::new()
        }
    }

    pub fn add<S: Into<String>>(&mut self, id: S, texture: Texture2D) -> Rc<Texture2D> {
        let t = Rc::new(texture);
        self.registry.insert(id.into(), t.clone());
        t
    }

    pub fn remove<S: Into<String>>(&mut self, id: S) -> Option<Rc<Texture2D>> {
        self.registry.remove(&id.into())
    }

    pub fn has<S: Into<String>>(&self, id: S) -> bool {
        self.registry.contains_key(&id.into())
    }

    pub fn get<S: Into<String>>(&self, id: S) -> Option<Rc<Texture2D>> {
        self.registry.get(&id.into()).cloned()
    }
}
