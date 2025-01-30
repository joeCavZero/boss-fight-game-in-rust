use super::engine::{Engine, RenderTextureModeDrawHandle};

pub trait Object {
    fn get_base_object(&mut self) -> &mut BaseObject;
    fn init(&mut self, engine: &mut Engine);
    fn update(&mut self, engine: &mut Engine);
    fn render(&self, d: &mut RenderTextureModeDrawHandle<'_> );
}

pub struct BaseObject {
    pub id: u32,
}

impl BaseObject {
    pub fn new() -> BaseObject {
        BaseObject {
            id: 0,
        }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn set_id(&mut self, id: u32) {
        self.id = id;
    }
}
