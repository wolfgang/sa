use raylib::consts::*;

use crate::_tests::helpers::testable_game::TestableGame;

#[test]
fn pressing_space_spawn_bullet_in_front_of_player_ship() {
    let mut game = make_game();

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
fn wait_half_second_before_spawning_another_bullet() {
    let mut game = make_game();

    game.key_is_down(KEY_SPACE);
    game.tick();
    game.renders_frame(vec![
        "..........",
        "..........",
        "..........",
        "....11....",
        "...0000...",
    ]);

    game.key_is_down(KEY_LEFT);
    game.tick_times(29);

    game.renders_frame(vec![
        "..........",
        "..........",
        "..........",
        "....11....",
        "0000......",
    ]);

    game.key_is_up(KEY_LEFT);
    game.tick_times(1);

    game.renders_frame(vec![
        "..........",
        "..........",
        "..........",
        ".11.11....",
        "0000......",
    ])
}

fn make_game() -> TestableGame {
    TestableGame::init()
        .with_dimensions(10, 5)
        .with_fps(60)
        .with_ship_speed(120)
        .build()
}