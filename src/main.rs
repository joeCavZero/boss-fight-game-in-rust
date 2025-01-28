use proj04::{engine::{engine::*, scene::Scene}, game::scenes::scene1::Scene1};

fn main() {
    
    let mut engine = Engine::new(
        640, 360,
        "cool game",
    );
    engine.texture_manager.load_texture("player", "assets/images/player.png").unwrap();
    engine.texture_manager.load_texture("tileset1", "assets/images/tileset1.png").unwrap();

    let mut scene1 = Box::new(Scene1::new());
    
    scene1.init(&mut engine);
    
    engine.set_scene(scene1);
    engine.run();

}
