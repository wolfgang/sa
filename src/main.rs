use std::time::{SystemTime, UNIX_EPOCH};

use raylib::{Color, Rectangle, Vector2};
use raylib::consts::{KEY_LEFT, KEY_RIGHT};

fn main() {
    let rl = raylib::init()
        .size(1024, 700)
        .title("Staggered Assault")
        .build();

    let sprite_sheet = rl.load_texture("resources/spaceshooter_sheet.png");

    let source_rec = Rectangle {
        x: 325.0,
        y: 0.0,
        width: 98.0,
        height: 75.0,
    };

    let mut ship_pos = Vector2 { x: 100.0, y: 600.0 };

    let mut last_millis = get_now_millis();

    rl.set_target_fps(60);

    let mut last_seconds = rl.get_time();

    let speed = 240.0;

    while !rl.window_should_close() {
        let delta_millis = get_now_millis() - last_millis;
        let delta_seconds = (rl.get_time() - last_seconds);
        last_seconds = rl.get_time();
//        println!("{}", delta_seconds);
        last_millis = get_now_millis();
        if rl.is_key_down(KEY_LEFT as i32) {
            ship_pos.x -= 2.0;
        }

        if rl.is_key_down(KEY_RIGHT as i32) {
            ship_pos.x += 2.0;
        }

        rl.begin_drawing();
        rl.clear_background(Color::BLACK);
        rl.draw_text(&format!("{}", rl.get_fps()), 0, 0, 24, Color::RED);

        rl.draw_texture_rec(&sprite_sheet, source_rec, ship_pos, Color::WHITE);

        rl.end_drawing();
    }
}

fn get_now_millis() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64
}
