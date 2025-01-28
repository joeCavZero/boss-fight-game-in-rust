use std::{char, collections::HashMap, rc::Rc};

use raylib::prelude::*;

use super::engine::RenderTextureModeDrawHandle;

pub struct Tilemap {
    pub tiles: Vec<Vec<char>>,
    pub tile_size: f32,
    pub texture: Rc<Texture2D>,

    pub tileset: HashMap<char, Vector2>,
}

impl Tilemap {
    pub fn new(file_path: &str, tile_size: u32, texture: Rc<Texture2D>) -> Tilemap {
        let mut tl = Tilemap {
            tiles: Vec::new(),
            tile_size: tile_size as f32,
            texture: texture,
            tileset: HashMap::new(),
        };
        
        let content = match std::fs::read_to_string(file_path) {
            Ok(content) => content,
            Err(err) => {
                eprintln!("Error reading file {}: {}", file_path, err);
                String::new()
            }
            
        };

        tl.tiles.push(Vec::new());
        for c in content.chars() {
            if c == '\n' {
                tl.tiles.push(Vec::new());
            } else {
                let len = tl.tiles.len();
                tl.tiles[len - 1].push(c);
            }
        }

        tl
    }

    pub fn render(&mut self, d: &mut RenderTextureModeDrawHandle) {
        for (y, line) in self.tiles.iter().enumerate() {
            for (x, tile_char ) in line.iter().enumerate() {
                match self.tileset.get(tile_char) {
                    Some(pos) => {
                        d.draw_texture_pro(
                            &*self.texture,
                            Rectangle::new(
                                pos.x,
                                pos.y,
                                self.tile_size,
                                self.tile_size,
                            ),
                            Rectangle::new(
                                x as f32 * self.tile_size,
                                y as f32 * self.tile_size,
                                self.tile_size,
                                self.tile_size,
                            ),
                            Vector2::new(0.0, 0.0),
                            0.0,
                            Color::WHITE,
                        );
                    }
                    _ => { }
                };
                
            }
        }
    }

    pub fn add_tile_tileset(&mut self, tile_char: char, x: u32, y: u32) {
        self.tileset.insert(tile_char, Vector2::new(x as f32, y as f32));
    }
}