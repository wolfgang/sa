use raylib::consts::*;

use crate::_tests::helpers::testable_game::TestableGame;

#[test]
fn pressing_space_spawn_bullet_in_front_of_player_ship() {
    let mut game = TestableGame::init()
        .with_dimensions(10, 5)
        .with_fps(60)
        .with_ship_speed(120)
        .build();

    game.key_is_down(KEY_RIGHT);
    game.tick();

    game.key_is_up(KEY_RIGHT);
    game.key_is_down(KEY_SPACE);
    game.tick();

    game.renders_frame(vec![
        "..........",
        "..........",
        "..........",
        "......11..",
        ".....0000.",
    ])
}

#[test]
fn wait_half__second_before_spawning_another_bullet() {
    let mut game = TestableGame::init()
        .with_dimensions(10, 5)
        .with_fps(60)
        .with_ship_speed(120)
        .build();

    game.key_is_down(KEY_RIGHT);
    game.tick();

    game.key_is_up(KEY_RIGHT);
    game.key_is_down(KEY_SPACE);
    game.tick();

    game.renders_frame(vec![
        "..........",
        "..........",
        "..........",
        "......11..",
        ".....0000.",
    ])
}