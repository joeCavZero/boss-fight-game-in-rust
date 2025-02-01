use std::any::Any;

use raylib::{color::Color, math::{Rectangle, Vector2}, prelude::RaylibDraw};

use crate::{engine::{engine::{Engine, RenderTextureModeDrawHandle}, entity::Entity, object::Object}, math};

use super::enemy::Enemy;

pub struct Shoot {
    pub entity: Entity,
    pub speed: f32,
}

impl Shoot {
    pub fn new(x: f32, y: f32, angle_vector_x: f32, angle_vector_y: f32) -> Shoot {
        Shoot {
            entity: Entity::new("shoot".to_string(), x, y, 16.0, 4.0, angle_vector_x, angle_vector_y),
            speed: 5.0,
        }
    }
}

impl Object for Shoot {
    fn as_any(&mut self) -> &mut dyn Any {
        self
    }

    fn get_base_object(&mut self) -> &mut crate::engine::object::BaseObject {
        &mut self.entity.base_object
    }

    fn init(&mut self, _engine: &mut Engine) {
        
    }

    fn render(&self, _: &mut Engine, d: &mut RenderTextureModeDrawHandle<'_>) {
        d.draw_rectangle_pro(
            Rectangle::new(
                self.entity.position.x,
                self.entity.position.y,
                self.entity.size.x,
                self.entity.size.y,
            ),
            Vector2::new(self.entity.size.x / 2.0, self.entity.size.y / 2.0),
            math::vector2::vector2_to_angle(self.entity.motion),
            Color::YELLOW,
        );
    }

    fn update(&mut self, engine: &mut Engine) {
        if self.entity.position.x < -50.0 || self.entity.position.x > engine.canvas_size.0 as f32 + 50.0 || self.entity.position.y < -50.0 || self.entity.position.y > engine.canvas_size.1 as f32 + 50.0 {
            engine.get_scene().unwrap().get_base_scene().remove_object(self.get_base_object().get_id());
        }
        
        self.entity.position.x -= self.entity.motion.x * self.speed;
        self.entity.position.y -= self.entity.motion.y * self.speed;
        
        let mut objects_to_remove: Vec<u32> = Vec::new();
        for obj in engine.get_scene().unwrap().get_base_scene().objects.iter_mut() {
            if obj.get_base_object().get_name() == "boss" {
                // cast to enemy type
                if let Some(boss) = obj.as_any().downcast_mut::<Enemy>() {
                    if boss.entity.is_colliding_with_rect(&self.entity.get_rect()) {
                        const DAMAGE: i32 = 10;
                        
                        boss.health = std::cmp::max(0, boss.health - DAMAGE);
                        
                        objects_to_remove.push(self.get_base_object().get_id());
                    }
                }
            }
        }

        for id in objects_to_remove {
            engine.get_scene().unwrap().get_base_scene().remove_object(id);
        }
    }
}