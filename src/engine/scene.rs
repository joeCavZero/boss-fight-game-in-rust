use std::collections::HashMap;

use super::{engine::{Engine, RenderTextureModeDrawHandle}, object::Object, tilemap::Tilemap};

pub trait Scene {
    fn init(&mut self, engine: &mut Engine);
    fn update(&mut self, engine: &mut Engine);
    fn render(&mut self, d: &mut RenderTextureModeDrawHandle<'_>);  

    fn get_base_scene(&mut self) -> &mut BaseScene;
}

pub struct BaseScene {
    pub engine: Option<*mut Engine>,
    pub objects: Vec<Box<dyn Object>>,
    pub tilemaps: HashMap<String, Tilemap>,
    pub id_counter: u32,
}

impl BaseScene {
    pub fn new() -> BaseScene {
        BaseScene {
            engine: None,
            objects: Vec::new(),
            tilemaps: HashMap::new(),
            id_counter: 0,
        }
    }

    pub fn init(&mut self, engine: &mut Engine) {
        self.engine = Some(engine);
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
        obj.as_mut().get_base_object().set_id(self.id_counter);
        self.id_counter += 1;
        self.objects.push(obj);
    }

    pub fn add_tilemap(&mut self, name: &str, tilemap: Tilemap) {
        self.tilemaps.insert(name.to_string(), tilemap);
    }

    pub fn get_tilemap(&mut self, name: &str) -> Option<&mut Tilemap> {
        self.tilemaps.get_mut(name)
    }

    pub fn remove_object(&mut self, id: u32) {
        for i in 0..self.objects.len() {
            if self.objects[i].get_base_object().get_id() == id {
                self.objects.remove(i);
                break;
            }
        }
    }
}