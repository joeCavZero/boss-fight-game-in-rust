use std::any::Any;

use raylib::prelude::*;

use crate::engine::{engine::{Engine, RenderTextureModeDrawHandle}, entity::Entity, object::{BaseObject, Object}};

pub struct Enemy {
    pub entity: Entity,
    pub speed: f32,
    pub texture: Option<*mut Texture2D>,
    pub animation_frame: f32,
    pub animation_speed: f32,
    pub animation_quantity: u32,
    pub health: u32,
}

impl Enemy {
    pub fn new(x: f32, y: f32) -> Enemy {
        Enemy {
            entity: Entity::new("boss".to_string(), x, y, 64.0, 64.0, 1.0, 1.0),
            speed: 4.0,
            texture: None,
            animation_frame: 0.0,
            animation_speed: 0.01,
            animation_quantity: 2,
            health: 100,
        }
    }
}

impl Object for Enemy {
    fn as_any(&mut self) -> &mut dyn Any {
        self
    }

    fn get_base_object(&mut self) -> &mut BaseObject {
        &mut self.entity.base_object
    }

    fn init(&mut self, engine: &mut Engine) {
        self.texture = Some(engine.texture_manager.get_texture("enemy").unwrap());
    }
    fn update(&mut self, engine: &mut Engine) {

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

        if self.health > 0 {
            self.entity.position.x += self.entity.motion.x * self.speed;
            self.entity.position.y += self.entity.motion.y * self.speed;
        }
        self.animation_frame += self.animation_speed;
        if self.animation_frame >= self.animation_quantity as f32 {
            self.animation_frame = 0.0;
        }
    }

    fn render(&self, _: &mut Engine, d: &mut RenderTextureModeDrawHandle<'_> ) {

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

        d.draw_rectangle(
            (self.entity.position.x + self.entity.size.x / 2.0 - self.health as f32 / 2.0) as i32,
            (self.entity.position.y - 10.0) as i32,
            self.health as i32,
            5,
            Color::RED,
        );
    }
}