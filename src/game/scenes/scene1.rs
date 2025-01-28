use crate::{engine::{engine::{Engine, RenderTextureModeDrawHandle}, scene::{BaseScene, Scene}, tilemap::Tilemap}, game::objects::{enemy::Enemy, player::Player}};
pub struct Scene1 {
    pub base_scene: BaseScene,
}

impl Scene1 {
    pub fn new() -> Scene1 {
        Scene1 {
            base_scene: BaseScene::new(),
        }
    }
    
}

impl Scene for Scene1 {
    fn init(&mut self, engine: &mut Engine) {
        self.base_scene.init(engine);

        self.base_scene.add_object(Box::new(Player::new(50.0, 50.0)));
        self.base_scene.add_object(Box::new(Enemy::new(100.0, 100.0)));
        
        let mut tilemap = Tilemap::new(
            "data/tilemap1.data",
            16,
            engine.texture_manager.get_texture("tileset1").unwrap(),
        );
        
        tilemap.add_tile_tileset('#', 0, 0);
        tilemap.add_tile_tileset('1', 16, 0);

        self.base_scene.add_tilemap(
            "tilemap",
            tilemap,
        );

    }

    fn update(&mut self, engine: &mut Engine) {
        self.base_scene.update(engine);
    }

    fn render(&mut self, d: &mut RenderTextureModeDrawHandle<'_>) {
        self.base_scene.render(d);
    }
}