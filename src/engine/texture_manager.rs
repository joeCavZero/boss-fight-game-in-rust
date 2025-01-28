use std::rc::Rc;

use raylib::prelude::*;

pub struct TextureManager {
    rl: *mut RaylibHandle,
    thread: *const RaylibThread,
    pub textures: std::collections::HashMap<String, Rc<Texture2D>>,
}

impl TextureManager {
    pub fn new( rl: *mut RaylibHandle, thread: *const RaylibThread) -> TextureManager {
        TextureManager {
            rl: rl,
            thread: thread,
            textures: std::collections::HashMap::new(),
        }
    }

    pub fn load_texture(&mut self, name: &str, path: &str) -> Result<(), String> {
        let rl = unsafe { &mut *self.rl };
        let thread = unsafe { &*self.thread };
        match rl.load_texture(thread, path) {
            Ok(texture) => {
                self.textures.insert(name.to_string(), Rc::new(texture));
                Ok(())
            },
            Err(e) => {
                let err = format!("---> Error loading texture {}: {}", path, e);
                eprintln!("{}", err);
                Err(err)
            },
        }
    }

    pub fn get_texture(&self, name: &str) -> Result<Rc<Texture2D>, String> {
        match self.textures.get(&name.to_string()) {
            Some(texture) => {
                let aux = Rc::clone(texture);
                println!(
                    "---> There are {} references pointing to the texture", 
                    Rc::strong_count(texture)
                );
                Ok(aux)
            }
            None => {
                let err = format!("---> Texture {} has not been loaded", name);
                eprintln!("{}", err);
                Err(err)
            },
        }
    }
}