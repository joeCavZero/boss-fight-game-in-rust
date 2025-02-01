use crate::{engine::{engine::{Engine, RenderTextureModeDrawHandle}, scene::{BaseScene, Scene}, tilemap::Tilemap, timer::Timer}, game::objects::{enemy::Enemy, player::Player}};

use super::menu::Menu;

pub struct Scene1 {
    base_scene: BaseScene,
    boss_death_timer: Timer,
    scene_changing_timer: Timer,
    is_boss_dead: bool,
    is_player_dead: bool,
    boss_removed: bool,
}

impl Scene1 {
    pub fn new() -> Scene1 {
        Scene1 {
            base_scene: BaseScene::new(),
            boss_death_timer: Timer::new(),
            scene_changing_timer: Timer::new(),
            is_boss_dead: false,
            is_player_dead: false,
            boss_removed: false,
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

        self.base_scene.add_tilemap("main-tilemap", tilemap);
    }

    fn update(&mut self, engine: &mut Engine) {
        self.base_scene.update(engine);

        let mut boss_id: Option<u32> = None;

        if let Some(obj) = self.base_scene.get_object_by_name("boss") {
            if let Some(boss) = obj.as_any().downcast_mut::<Enemy>() {
                if boss.health <= 0 {
                    if !self.is_boss_dead {
                        self.is_boss_dead = true;
                        self.boss_death_timer.reset(5.0);
                    }
                    if self.is_boss_dead && self.boss_death_timer.is_ready() && !self.boss_removed {
                        boss_id = Some(obj.get_base_object().get_id());
                    }
                }
            }
        }

        if let Some(id) = boss_id {
            self.base_scene.remove_object(id);
            self.boss_removed = true; // Marca que o boss foi removido
            self.scene_changing_timer.reset(10.0);
        }

        if self.boss_removed && self.scene_changing_timer.is_ready() {
            let menu_scene = Box::new(Menu::new());
            engine.set_scene(menu_scene);
        }

        if let Some(obj) = self.base_scene.get_object_by_name("player") {
            if let Some(player) = obj.as_any().downcast_mut::<Player>() {
                if player.health <= 0 {
                    if !self.is_player_dead {
                        self.is_player_dead = true;
                        self.scene_changing_timer.reset(5.0);
                    }
                    if self.scene_changing_timer.is_ready() {
                        let menu_scene = Box::new(Menu::new());
                        engine.set_scene(menu_scene);
                    }

                }

                
            }
        }
    }

    fn render(&mut self, engine: &mut Engine, d: &mut RenderTextureModeDrawHandle<'_>) {
        self.base_scene.render(engine, d);
    }
}
