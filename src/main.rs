use raylib::{Color, Rectangle, Vector2};

fn main() {
    let rl = raylib::init()
        .size(800, 600)
        .title("Staggered Assault")
        .build();

    let ship_texture = rl.load_texture("resources/ship.png");

    let source_rec = Rectangle {
        x: 0.0,
        y: 0.0,
        width: 32.0,
        height: 42.0,
    };

    let dest_rec = Rectangle {
        x: 0.0,
        y: 600.0 - 84.0,
        width: 32.0,
        height: 42.0,
    };

    while !rl.window_should_close() {
        rl.begin_drawing();
        rl.clear_background(Color::BLACK);

        rl.draw_texture_pro(&ship_texture, source_rec, dest_rec, Vector2::zero(), 0.0, Color::WHITE);

        rl.end_drawing();
    }
}
