use std::any::Any;

use super::engine::{Engine, RenderTextureModeDrawHandle};

pub trait Object {
    fn as_any(&mut self) -> &mut dyn Any;
    fn get_base_object(&mut self) -> &mut BaseObject;
    fn init(&mut self, engine: &mut Engine);
    fn update(&mut self, engine: &mut Engine);
    fn render(&self, engine: &mut Engine, d: &mut RenderTextureModeDrawHandle<'_> );
}

pub struct BaseObject {
    pub id: u32,
    pub name: String,
}

impl BaseObject {
    pub fn new(name: String) -> BaseObject {
        BaseObject {
            id: 0,
            name: name,
        }
    }
    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn set_id(&mut self, id: u32) {
        self.id = id;
    }
}
