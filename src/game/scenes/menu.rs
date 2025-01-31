use raylib::prelude::*;

use crate::engine::{engine::{Engine, RenderTextureModeDrawHandle}, scene::{BaseScene, Scene}};

use super::scene1::Scene1;

pub struct Menu {
    base_scene: BaseScene,
}

impl Menu {
    pub fn new() -> Menu {
        Menu {
            base_scene: BaseScene::new(),
        }
    }
}

impl Scene for Menu {
    
    fn get_base_scene(&mut self) -> &mut BaseScene {
        &mut self.base_scene
    }
    fn init(&mut self, engine: &mut Engine) {
        self.base_scene.init(engine);
    }

    fn update(&mut self, engine: &mut Engine) {
        self.base_scene.update(engine);

        if engine.is_action_just_pressed("start") {
            let mut new_scene1 = Box::new(Scene1::new());
            new_scene1.init(engine);

            engine.set_scene(new_scene1);
        }
    }

    fn render(&mut self, engine: &mut Engine, d: &mut RenderTextureModeDrawHandle<'_>) {
        self.base_scene.render(engine, d);

        d.draw_text(
            "Press Enter to start", 
            25, 25, 
            20, 
            Color::WHITE,
        );
        const PADDING: i32 = 10;
        d.draw_rectangle_lines(
            PADDING, PADDING,
            engine.canvas_size.0 as i32 - PADDING * 2,
            engine.canvas_size.1 as i32 - PADDING * 2,
            Color::WHITE,
        );
    }
}