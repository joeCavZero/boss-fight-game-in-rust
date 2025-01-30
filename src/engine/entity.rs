use raylib::prelude::*;

use super::{object::BaseObject, tilemap::Tilemap};

pub struct Entity {
    pub base_object: BaseObject,
    pub position: Vector2,
    pub size: Vector2,
    pub motion: Vector2,
}

impl Entity {
    pub fn new(x: f32, y: f32, width: f32, height: f32, motion_x: f32, motion_y: f32) -> Entity {
        Entity {
            base_object: BaseObject::new(),
            position: Vector2::new(x, y),
            size: Vector2::new(width, height),
            motion: Vector2::new(motion_x, motion_y),
        }
    }

    pub fn is_colliding_with_tilemap(&self, tilemap: &Tilemap) -> bool {
        let x = self.position.x / tilemap.tile_size;
        let y = self.position.y / tilemap.tile_size;

        let x_end = (self.position.x + self.size.x) / tilemap.tile_size;
        let y_end = (self.position.y + self.size.y) / tilemap.tile_size;

        for i in x as usize..x_end as usize {
            for j in y as usize..y_end as usize {
                if tilemap.tiles[j][i] != ' ' {
                    return true;
                }
            }
        }

        false
    }

    pub fn move_and_collide(&mut self,speed: f32, tilemap: &mut Tilemap) {
        let mut is_colliding = false;
        let new_pos = Vector2::new(
            self.position.x + self.motion.x * speed,
            self.position.y + self.motion.y * speed,
        );

        for i_y in 0..tilemap.tiles.len() {
            for i_x in 0..tilemap.tiles[i_y].len() {
                let tile_char = tilemap.tiles[i_y][i_x];

                

                if tile_char != ' ' {
                    let tile_pos = Vector2::new(
                        i_x as f32 * tilemap.tile_size,
                        i_y as f32 * tilemap.tile_size,
                    );

                    let tile_rect = Rectangle::new(
                        tile_pos.x,
                        tile_pos.y,
                        tilemap.tile_size,
                        tilemap.tile_size,
                    );

                    let entity_rect = Rectangle::new(
                        new_pos.x,
                        new_pos.y,
                        self.size.x,
                        self.size.y,
                    );

                    if tile_rect.check_collision_recs(&entity_rect) {
                        is_colliding = true;
                    }
                }
            }
        }
        if is_colliding == false {
            self.position = new_pos;
        }
    }
}