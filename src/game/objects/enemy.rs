use std::rc::Rc;

use raylib::prelude::*;

use crate::engine::{entity::Entity, object::Object};

pub struct Enemy {
    pub entity: Entity,
    pub speed: f32,

    pub texture: Option<Rc<Texture2D>>,
}

impl Enemy {
    pub fn new(x: f32, y: f32) -> Enemy {
        Enemy {
            entity: Entity::new(x, y, 48.0, 64.0),
            speed: 4.0,

            texture: None,
        }
    }
}

impl Object for Enemy {
    fn init(&mut self, engine: &mut crate::engine::engine::Engine) {
        self.texture = Some(engine.texture_manager.get_texture("player").unwrap());
    }
    fn update(&mut self, engine: &mut crate::engine::engine::Engine) {

        let random_number: f32 = rand::random::<f32>();
        
        let aux = if random_number < 0.3 {
            -1.0
        } else if random_number < 0.6 {
            0.0
        } else {
            1.0
        };

        if self.entity.position.x < 0.0 {
            self.entity.motion.x = 1.0;
            self.entity.motion.y = aux;
        } else if self.entity.position.x + self.entity.size.x > engine.canvas_size.0 as f32 {
            self.entity.motion.x = -1.0;
            self.entity.motion.y = aux;
        }
        
        if self.entity.position.y < 0.0 {
            self.entity.motion.y = 1.0;
            self.entity.motion.x = aux;
        } else if self.entity.position.y + self.entity.size.y > engine.canvas_size.1 as f32 {
            self.entity.motion.y = -1.0;
            self.entity.motion.x = aux;
        }

        self.entity.motion.normalize();
        self.entity.position.x += self.entity.motion.x * self.speed;
        self.entity.position.y += self.entity.motion.y * self.speed;

    }

    fn render(&self, d: &mut crate::engine::engine::RenderTextureModeDrawHandle<'_> ) {
        d.draw_rectangle_v(
            self.entity.position,
            self.entity.size,
            Color::RED,
        );

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