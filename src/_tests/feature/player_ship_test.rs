use raylib::consts::*;

use crate::_tests::helpers::testable_game::TestableGame;

#[test]
fn render_player_ship_at_bottom_of_screen_centered() {
    let mut game = TestableGame::init()
        .with_dimensions(10, 5)
        .build();

    game.renders_frame(vec![
        "..........",
        "..........",
        "..........",
        "..........",
        "...0000...",
    ])
}

#[test]
fn ship_movement() {
    let mut game = TestableGame::init()
        .with_dimensions(10, 2)
        .with_fps(60)
        .with_ship_speed(120)
        .build();

    game.renders_frame(vec![
        "..........",
        "...0000...",
    ]);

    game.key_is_down(KEY_LEFT);
    game.tick();

    game.renders_frame(vec![
        "..........",
        ".0000.....",
    ]);


    game.key_is_up(KEY_LEFT);
    game.key_is_down(KEY_RIGHT);
    game.tick();
    game.tick();

    game.renders_frame(vec![
        "..........",
        ".....0000.",
    ]);

    game.key_is_up(KEY_RIGHT);
    game.tick();
    game.tick();

    game.renders_frame(vec![
        "..........",
        ".....0000.",
    ]);
}

#[test]
fn ship_collides_with_edges() {
    let mut game = TestableGame::init()
        .with_dimensions(10, 2)
        .with_fps(60)
        .with_ship_speed(120)
        .build();

    game.key_is_down(KEY_LEFT);
    game.tick();
    game.tick();

    game.renders_frame(vec![
        "..........",
        "0000......",
    ]);

    game.key_is_up(KEY_LEFT);
    game.key_is_down(KEY_RIGHT);
    game.tick();
    game.tick();
    game.tick();
    game.tick();

    game.renders_frame(vec![
        "..........",
        "......0000",
    ]);
}