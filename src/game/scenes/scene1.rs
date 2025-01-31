use crate::{engine::{engine::{Engine, RenderTextureModeDrawHandle}, scene::{BaseScene, Scene}, tilemap::Tilemap, timer::Timer}, game::objects::{enemy::Enemy, player::Player}};
pub struct Scene1 {
    base_scene: BaseScene,
    boss_death_timer: Timer,
    is_boss_dead: bool,
}

impl Scene1 {
    pub fn new() -> Scene1 {
        Scene1 {
            base_scene: BaseScene::new(),
            boss_death_timer: Timer::new(5.0),
            is_boss_dead: false,
        }
    }
    
}

impl Scene for Scene1 {
    
    fn get_base_scene(&mut self) -> &mut BaseScene {
        &mut self.base_scene
    }
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
        tilemap.add_tile_tileset('2', 0, 16);
        tilemap.add_tile_tileset('3', 16, 16);

        self.base_scene.add_tilemap(
            "main-tilemap",
            tilemap,
        );

    }

    fn update(&mut self, engine: &mut Engine) {
        self.base_scene.update(engine);
        
        let mut boss_id: Option<u32> = None;

        if let Some(obj) = self.base_scene.get_object_by_name("boss") {
            if let Some(boss) = obj.as_any().downcast_mut::<Enemy>() {
                if boss.health <= 0 {
                    if self.is_boss_dead == false {
                        self.is_boss_dead = true;
                        self.boss_death_timer.reset();
                    }
                    if self.is_boss_dead && self.boss_death_timer.is_ready() {
                        boss_id = Some(obj.get_base_object().get_id());
                    }
                }
            }
        }
        
        
        if let Some(id) = boss_id {
            self.base_scene.remove_object(id);
        }
    }

    fn render(&mut self, engine: &mut Engine, d: &mut RenderTextureModeDrawHandle<'_>) {
        self.base_scene.render(engine, d);
    }

}