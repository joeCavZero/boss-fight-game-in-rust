use raylib::prelude::*;

use crate::engine::{entity::Entity, object::Object};

pub struct Enemy {
    pub entity: Entity,
    pub speed: f32,
    pub texture: Option<*mut Texture2D>,
    pub animation_frame: f32,
    pub animation_speed: f32,
    pub animation_quantity: u32,
}

impl Enemy {
    pub fn new(x: f32, y: f32) -> Enemy {
        Enemy {
            entity: Entity::new(x, y, 32.0, 32.0, 1.0, 1.0),
            speed: 4.0,
            texture: None,
            animation_frame: 0.0,
            animation_speed: 0.01,
            animation_quantity: 2,
        }
    }
}

impl Object for Enemy {
    fn init(&mut self, engine: &mut crate::engine::engine::Engine) {
        self.texture = Some(engine.texture_manager.get_texture("enemy").unwrap());
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

        self.animation_frame += self.animation_speed;
        if self.animation_frame >= self.animation_quantity as f32 {
            self.animation_frame = 0.0;
        }
    }

    fn render(&self, d: &mut crate::engine::engine::RenderTextureModeDrawHandle<'_> ) {

        d.draw_text(
            format!("animation_frame: {}", self.animation_frame.round()).as_str(),
            10, 10,
            30,
            Color::WHITE,
        );
        
        let texture = unsafe { &*self.texture.unwrap() };
        d.draw_texture_pro(
            texture,
            Rectangle::new(
                32.0 * self.animation_frame.round(),0.0,
                32.0, 32.0,
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