use std::fs;
use std::rc::Rc;

use sa::game::Game;
use sa::raylib::{input::RaylibInput, renderer::RaylibRenderer};
use sa::raylib::sprite_registry::SpriteRegistry;

fn main() {
    let screen_width = 650;
    let screen_height = 800;
    let fps = 60;

    let rl = Rc::new(
        raylib::init()
            .size(screen_width, screen_height)
            .title("Staggered Assault")
            .build());

    let sprite_xml = fs::read_to_string("resources/spaceshooter_sheet.xml").unwrap();
    let sprite_registry = SpriteRegistry::from_xml(sprite_xml);

    let ship_rec = sprite_registry.get_source_rec("playerShip3_red.png");
    let bullet_rec = sprite_registry.get_source_rec("laserBlue08.png");
    let enemy_rec = sprite_registry.get_source_rec("enemyBlack2.png");

    let mut game = Game::init()
        .with_input(RaylibInput::new_rc(rl.clone()))
        .with_dimensions(screen_width as u32, screen_height as u32)
        .with_ship_dimensions(ship_rec.width as u32, ship_rec.height as u32)
        .with_bullet_dimensions(bullet_rec.width as u32, bullet_rec.height as u32)
        .with_fps(fps)
        .with_ship_speed(360)
        .with_bullet_speed(540)
        .with_enemy_speed(60, 30)
        .build();

    let mut renderer = RaylibRenderer::new(rl.clone());
    renderer.register_sprite(0, ship_rec);
    renderer.register_sprite(1, bullet_rec);
    renderer.register_sprite(2, enemy_rec);

    let frame_time = 1.0 / fps as f64;
    let mut current_delta = 0.0;
    let mut last_time = rl.get_time();

    while !rl.window_should_close() {
        let time = rl.get_time();
        current_delta += time - last_time;
        last_time = time;
        while current_delta >= frame_time {
            current_delta -= frame_time;
            game.tick();
            rl.begin_drawing();
            game.render(&mut renderer);
            rl.end_drawing();
        }
    }
}
