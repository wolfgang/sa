use raylib::{Color, Rectangle, Vector2};

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


    while !rl.window_should_close() {
        rl.begin_drawing();
        rl.clear_background(Color::BLACK);

        rl.draw_texture_rec(&sprite_sheet, source_rec, Vector2 { x: 100.0, y: 700.0 - 100.0 }, Color::WHITE);

        rl.end_drawing();
    }
}
