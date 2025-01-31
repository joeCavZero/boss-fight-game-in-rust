use proj04::{engine::{engine::*, scene::Scene}, game::scenes::menu::Menu};

fn main() {
    
    let mut engine = Engine::new(
        640, 360,
        "cool game",
    );
    engine.texture_manager.load_texture("player", "assets/images/player.png").unwrap();
    engine.texture_manager.load_texture("enemy", "assets/images/enemy.png").unwrap();
    engine.texture_manager.load_texture("tileset1", "assets/images/tileset1.png").unwrap();

    let mut menu = Box::new(Menu::new());
    
    menu.init(&mut engine);
    
    engine.set_scene(menu);
    
    engine.run();

}
