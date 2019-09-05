use std::rc::Rc;

use raylib::Color;

use sa::game::game::Game;
use sa::gfx::raylib_renderer::RaylibRenderer;

fn main() {
    let rl = Rc::new(
        raylib::init()
            .size(1024, 700)
            .title("Staggered Assault")
            .build());


    let game = Game::init()
        .with_dimensions(1024, 700)
        .with_ship_dimensions(98, 75)
        .build();

    let mut renderer = RaylibRenderer::new(rl.clone());

    let fps = 60.0;
    rl.set_target_fps(fps as i32);

    while !rl.window_should_close() {
        rl.begin_drawing();
        rl.clear_background(Color::BLACK);
        game.render(&mut renderer);
        rl.end_drawing();
    }
}
