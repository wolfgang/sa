use raylib::consts::*;

use crate::_tests::helpers::testable_game::TestableGame;

#[test]
fn move_ship_left_right_according_to_input() {
    let mut game = TestableGame::init()
        .with_dimensions(10, 2)
        .with_ship_speed(1)
        .build();

    game.renders_frame(vec![
        "..........",
        "...0000..."
    ]);

    game.key_is_down(KEY_LEFT);
    game.tick();
    game.renders_frame(vec![
        "..........",
        "..0000...."
    ]);

    game.key_is_up(KEY_LEFT);
    game.tick();
    game.renders_frame(vec![
        "..........",
        "..0000...."
    ]);

    game.key_is_down(KEY_RIGHT);
    game.tick();
    game.renders_frame(vec![
        "..........",
        "...0000..."
    ]);
}

#[test]
fn ship_collides_with_screen_bounds() {
    let mut game = TestableGame::init()
        .with_dimensions(10, 2)
        .with_ship_speed(1)
        .build();

    game.renders_frame(vec![
        "..........",
        "...0000..."
    ]);


    game.key_is_down(KEY_LEFT);
    game.tick_times(10);
    game.renders_frame(vec![
        "..........",
        "0000......"
    ]);

    game.key_is_up(KEY_LEFT);
    game.key_is_down(KEY_RIGHT);
    game.tick_times(10);
    game.renders_frame(vec![
        "..........",
        "......0000"
    ]);
}