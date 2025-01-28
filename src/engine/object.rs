use super::engine::{Engine, RenderTextureModeDrawHandle};

pub trait Object {
    fn init(&mut self, engine: &mut Engine);
    fn update(&mut self, engine: &mut Engine);
    fn render(&self, d: &mut RenderTextureModeDrawHandle<'_> );
}

