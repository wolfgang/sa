use std::rc::Rc;

use raylib::Rectangle;

use sa::game::game::Game;
use sa::raylib_input::RaylibInput;
use sa::raylib_renderer::RaylibRenderer;

const SCREEN_WIDTH: u32 = 400;
const SCREEN_HEIGHT: u32 = 640;

fn main() {
    let rl = Rc::new(raylib::init()
        .size(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32)
        .title("Staggered Assault")
        .build());

    let mut renderer = RaylibRenderer::new(&rl);
    let player_ship_rec = Rectangle {
        x: 325.0,
        y: 0.0,
        width: 98.0,
        height: 75.0,
    };

    renderer.register_sprite(0, player_ship_rec);

    let mut game = Game::init()
        .with_dimensions(SCREEN_WIDTH, SCREEN_HEIGHT)
        .with_ship_dimensions(player_ship_rec.width as u32, player_ship_rec.height as u32)
        .with_input(RaylibInput::new_rc(rl.clone()))
        .with_ship_speed(4)
        .build();

    let fps = 60;

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
