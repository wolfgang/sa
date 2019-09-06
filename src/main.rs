use std::rc::Rc;

use raylib::{Color, Rectangle};

use sa::game::game::Game;
use sa::gfx::input::RaylibInput;
use sa::gfx::renderer::RaylibRenderer;

fn main() {
    let screen_width = 400;
    let screen_height = 800;
    let ship_width = 98;
    let ship_height = 75;
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
        .with_fps(fps)
        .with_ship_speed(300)
        .build();

    let mut renderer = RaylibRenderer::new(rl.clone());

    let ship_rec = Rectangle {
        x: 325.0,
        y: 0.0,
        width: ship_width as f32,
        height: ship_height as f32,
    };

    renderer.register_sprite(0, ship_rec);

    rl.set_target_fps(fps as i32);

    while !rl.window_should_close() {
        game.tick();
        rl.begin_drawing();
        rl.clear_background(Color::BLACK);
        game.render(&mut renderer);
        rl.end_drawing();
    }
}
