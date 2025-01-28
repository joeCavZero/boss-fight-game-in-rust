use std::collections::HashMap;

use super::{engine::{Engine, RenderTextureModeDrawHandle}, object::Object, tilemap::Tilemap};

pub trait Scene {
    fn init(&mut self, engine: &mut Engine);
    fn update(&mut self, engine: &mut Engine);
    fn render(&mut self, d: &mut RenderTextureModeDrawHandle<'_>);  
}

pub struct BaseScene {
    pub engine: Option<*mut Engine>,
    pub objects: Vec<Box<dyn Object>>,
    pub tilemaps: HashMap<String, Tilemap>,
}

impl BaseScene {
    pub fn new() -> BaseScene {
        BaseScene {
            engine: None,
            objects: Vec::new(),
            tilemaps: HashMap::new(),
        }
    }

    pub fn init(&mut self, engine: &mut Engine) -> &mut BaseScene {
        self.engine = Some(engine);
        self
    }

    pub fn update(&mut self, engine: &mut Engine) {
        for obj in self.objects.iter_mut() {
            obj.update(engine);
        }
    }

    pub fn render(&mut self, d: &mut RenderTextureModeDrawHandle<'_>) {
        for tl in self.tilemaps.values_mut() {
            tl.render(d);
        }
        
        for obj in self.objects.iter() {
            obj.render(d);
        }
        
    }

    pub fn add_object(&mut self, mut obj: Box<dyn Object>) {
        
        unsafe { obj.init(&mut *self.engine.unwrap()) }

        self.objects.push(obj);
    }

    pub fn add_tilemap(&mut self, name: &str, tilemap: Tilemap) {
        self.tilemaps.insert(name.to_string(), tilemap);
    }
}