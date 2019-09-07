use std::rc::Rc;

use raylib::Rectangle;

use sa::game::Game;
use sa::raylib::{input::RaylibInput, renderer::RaylibRenderer};

fn main() {
    let screen_width = 400;
    let screen_height = 800;
    let ship_width = 98;
    let ship_height = 75;
    let bullet_width = 48;
    let bullet_height = 46;
    let fps = 60;

    let rl = Rc::new(
        raylib::init()
            .size(screen_width, screen_height)
            .title("Staggered Assault")
            .build());


    let mut game = Game::init()
        .with_input(RaylibInput::new_rc(rl.clone()))
        .with_dimensions(screen_width as u32, screen_height as u32)
        .with_ship_dimensions(ship_width, ship_height)
        .with_bullet_dimensions(bullet_width, bullet_height)
        .with_fps(fps)
        .with_ship_speed(300)
        .with_bullet_speed(500)
        .build();

    let mut renderer = RaylibRenderer::new(rl.clone());

    let ship_rec = Rectangle {
        x: 325.0,
        y: 0.0,
        width: ship_width as f32,
        height: ship_height as f32,
    };

    let bullet_rec = Rectangle {
        x: 596.0,
        y: 961.0,
        width: bullet_width as f32,
        height: bullet_height as f32,
    };

    renderer.register_sprite(0, ship_rec);
    renderer.register_sprite(1, bullet_rec);

    rl.set_target_fps(fps as i32);

    while !rl.window_should_close() {
        game.tick();
        rl.begin_drawing();
        game.render(&mut renderer);
        rl.end_drawing();
    }
}
