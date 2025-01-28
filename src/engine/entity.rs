use raylib::prelude::*;

use super::tilemap::Tilemap;

pub struct Entity {
    pub position: Vector2,
    pub size: Vector2,
    pub motion: Vector2,
}

impl Entity {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Entity {
        Entity {
            position: Vector2::new(x, y),
            size: Vector2::new(width, height),
            motion: Vector2::new(0.0, 0.0),
        }
    }

    pub fn move_and_collide(&mut self, tilemap: &mut Tilemap) {
        let new_position = self.position + self.motion;

        let x = new_position.x;
        let y = new_position.y;

        let x1 = x;
        let y1 = y;
        let x2 = x + self.size.x;
        let y2 = y + self.size.y;

        let tile_size = tilemap.tile_size as f32;

        let mut x1_tile = (x1 / tile_size) as i32;
        let mut y1_tile = (y1 / tile_size) as i32;
        let mut x2_tile = (x2 / tile_size) as i32;
        let mut y2_tile = (y2 / tile_size) as i32;

        if x1_tile < 0 {
            x1_tile = 0;
        }
        if y1_tile < 0 {
            y1_tile = 0;
        }
        if x2_tile >= tilemap.tiles[0].len() as i32 {
            x2_tile = tilemap.tiles[0].len() as i32 - 1;
        }
        if y2_tile >= tilemap.tiles.len() as i32 {
            y2_tile = tilemap.tiles.len() as i32 - 1;
        }

        let mut collision = false;

        for y in y1_tile..=y2_tile {
            for x in x1_tile..=x2_tile {
                if tilemap.tiles[y as usize][x as usize] == '#' {
                    collision = true;
                    break;
                }
            }
        }

        if !collision {
            self.position = new_position;
        }
    }
}