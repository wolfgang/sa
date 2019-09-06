use std::rc::Rc;

use raylib::{Color, Rectangle};

use sa::game::game::Game;
use sa::game::raylib_input::RaylibInput;
use sa::gfx::raylib_renderer::RaylibRenderer;

fn main() {
    let rl = Rc::new(
        raylib::init()
            .size(1024, 700)
            .title("Staggered Assault")
            .build());


    let mut game = Game::init()
        .with_input(RaylibInput::new_rc(rl.clone()))
        .with_dimensions(1024, 700)
        .with_ship_dimensions(98, 75)
        .with_fps(60)
        .with_ship_speed(300)
        .build();

    let mut renderer = RaylibRenderer::new(rl.clone());

    let ship_rec = Rectangle {
        x: 325.0,
        y: 0.0,
        width: 98.0,
        height: 75.0,
    };

    renderer.register_sprite(0, ship_rec);



    let fps = 60.0;
    rl.set_target_fps(fps as i32);

    while !rl.window_should_close() {
        game.tick();
        rl.begin_drawing();
        rl.clear_background(Color::BLACK);
        game.render(&mut renderer);
        rl.end_drawing();
    }
}
