use raylib::{Color, Rectangle};

use sa::game::game::Game;
use sa::raylib_renderer::RaylibRenderer;

const SCREEN_WIDTH: u32 = 400;
const SCREEN_HEIGHT: u32 = 640;

fn main() {
    let rl = raylib::init()
        .size(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32)
        .title("Staggered Assault")
        .build();

    let mut renderer = RaylibRenderer::new(&rl);
    let player_ship_rec = Rectangle {
        x: 325.0,
        y: 0.0,
        width: 98.0,
        height: 75.0,
    };

    renderer.register_sprite(0, player_ship_rec);

    let game = Game::init()
        .with_dimensions(SCREEN_WIDTH, SCREEN_HEIGHT)
        .with_ship_dimensions(player_ship_rec.width as u32, player_ship_rec.height as u32)
        .build();


    while !rl.window_should_close() {
        rl.begin_drawing();
        rl.clear_background(Color::BLACK);
        game.render(&mut renderer);
        rl.end_drawing();
    }
}
