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

    let fps = 60.0;
    let speed = 600.0;

    rl.set_target_fps(fps as i32);

    let mut last_frame_time = 0.0;
    let mut last_info_time = 0.0;


    while !rl.window_should_close() {
        if rl.is_key_down(KEY_LEFT as i32) {
            ship_pos.x -= speed / fps;
        }

        if rl.is_key_down(KEY_RIGHT as i32) {
            ship_pos.x += speed / fps;
        }

        if rl.get_time() - last_info_time > 0.5 {
            last_info_time = rl.get_time();
            last_frame_time = rl.get_frame_time();

        }

        rl.begin_drawing();
        rl.clear_background(Color::BLACK);
        rl.draw_text(&format!("Frame time: {} Effective FPS: {}", last_frame_time, 1.0 / last_frame_time), 10, 10, 20, Color::RED);

        rl.draw_circle_v(Vector2 { x: ship_pos.x + 50.0, y: ship_pos.y - 100.0 }, 50.0, Color::MAROON);
        rl.draw_texture_rec(&sprite_sheet, source_rec, ship_pos, Color::WHITE);

        rl.end_drawing();
    }
}
