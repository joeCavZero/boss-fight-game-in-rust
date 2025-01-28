use std::rc::Rc;

use raylib::prelude::*;

use crate::engine::{engine::Engine, entity::Entity, object::Object};

pub struct Player {
    pub entity: Entity,
    pub speed: f32,

    pub texture: Option<Rc<Texture2D>>,
}

impl Player {
    pub fn new(x: f32, y: f32) -> Player {
        Player {
            entity: Entity::new(x, y, 48.0, 64.0),
            speed: 3.0,
            texture: None,
        }
    }
}

impl Object for Player {
    fn init(&mut self, engine: &mut Engine) {
        
        self.texture = Some(engine.texture_manager.get_texture("player").unwrap());

    }
    fn update(&mut self, engine: &mut Engine) {
        if engine.is_action_down("up") {
            self.entity.motion.y = -1.0;
        } else if engine.is_action_down("down") {
            self.entity.motion.y = 1.0;
        } else {
            self.entity.motion.y = 0.0;
        }

        if engine.is_action_down("right") {
            self.entity.motion.x = 1.0;
        } else if engine.is_action_down("left") {
            self.entity.motion.x = -1.0;
        } else {
            self.entity.motion.x = 0.0;
        }

        self.entity.motion.normalize();

        self.entity.position.x += self.entity.motion.x * self.speed;
        self.entity.position.y += self.entity.motion.y * self.speed;
    }

    fn render(&self, d: &mut RaylibTextureMode<'_, RaylibDrawHandle<'_>>) {
        
        
        d.draw_texture_pro(
            &**self.texture.as_ref().unwrap(),
            Rectangle::new(
                0.0,0.0,
                16.0, 24.0,
            ),
            Rectangle::new(
                self.entity.position.x,
                self.entity.position.y,
                self.entity.size.x,
                self.entity.size.y,
            ),
            Vector2::new(0.0,0.0),
            0.0,
            Color::WHITE,
        );
        
    }
}