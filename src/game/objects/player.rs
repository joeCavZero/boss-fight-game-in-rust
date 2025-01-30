use raylib::prelude::*;

use crate::{engine::{engine::Engine, entity::Entity, object::{BaseObject, Object}}, math};

use super::shoot::Shoot;

pub struct Player {
    pub entity: Entity,
    pub speed: f32,

    pub texture: Option<*mut Texture2D>,
    pub animation_frame: f32,
    pub animation_speed: f32,
    pub animation_quantity: u32,
}

impl Player {
    pub fn new(x: f32, y: f32) -> Player {
        Player {
            entity: Entity::new(x, y, 24.0, 32.0, 0.0, 0.0),
            speed: 3.0,
            texture: None,
            animation_frame: 0.0,
            animation_speed: 0.1,
            animation_quantity: 2,
        }
    }
}

impl Object for Player {
    fn get_base_object(&mut self) -> &mut BaseObject {
        &mut self.entity.base_object
    }

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

        let main_tilemap = engine.get_scene().unwrap().get_base_scene().get_tilemap("main-tilemap").unwrap();
        self.entity.move_and_collide(self.speed, main_tilemap);

        if self.entity.motion.x != 0.0 || self.entity.motion.y != 0.0 {
            self.animation_frame += self.animation_speed;
        }
        
        if self.animation_frame >= self.animation_quantity as f32 {
            self.animation_frame = 0.0;
        }

        // shoot
        if engine.is_action_just_pressed("z") {
            let mouse_position = engine.get_mouse_position();
            let aim_angle_vector = math::vector2::vector2_aim_to_vector2(
                self.entity.position,
                mouse_position,
            );
            engine.get_scene().unwrap().get_base_scene().add_object(
                Box::new(
                    Shoot::new(
                        self.entity.position.x+self.entity.size.x/2.0,
                        self.entity.position.y+self.entity.size.y/2.0,
                        aim_angle_vector.x,
                        aim_angle_vector.y,
                    ),
                ),
            );
        }
       
    }

    fn render(&self, d: &mut RaylibTextureMode<'_, RaylibDrawHandle<'_>>) {
        
        let texture = unsafe{ &*self.texture.unwrap() };
       
        d.draw_texture_pro(
            texture,
            Rectangle::new(
                24.0 * self.animation_frame.round(),
                0.0,
                24.0, 32.0,
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