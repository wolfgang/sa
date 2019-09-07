use raylib::consts::*;

use crate::_tests::helpers::testable_game::TestableGame;

#[test]
fn ship_movement() {
    let mut game = make_game();

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
    game.tick_twice();

    game.renders_frame(vec![
        "..........",
        ".....0000.",
    ]);

    game.key_is_up(KEY_RIGHT);
    game.tick_twice();

    game.renders_frame(vec![
        "..........",
        ".....0000.",
    ]);
}

#[test]
fn ship_collides_with_edges() {
    let mut game = make_game();

    game.key_is_down(KEY_LEFT);
    game.tick_twice();

    game.renders_frame(vec![
        "..........",
        "0000......",
    ]);

    game.key_is_up(KEY_LEFT);
    game.key_is_down(KEY_RIGHT);
    game.tick_times(4);

    game.renders_frame(vec![
        "..........",
        "......0000",
    ]);
}

fn make_game() -> TestableGame {
    TestableGame::init()
        .with_dimensions(10, 2)
        .build()
}