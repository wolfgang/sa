use raylib::consts::*;

use crate::_tests::helpers::testable_game::TestableGame;

#[test]
fn pressing_space_spawns_bullet_from_ship_center() {
    let mut game = TestableGame::init()
        .with_dimensions(10, 6)
        .with_bullet_speed(2)
        .with_bullet_dimensions(2, 1)
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
        "..........",
        ".....11...",
        "....0000..",
    ]);

    game.key_is_up(KEY_SPACE);

    game.tick();
    game.renders_frame(vec![
        "..........",
        "..........",
        ".....11...",
        "..........",
        "..........",
        "....0000..",
    ]);

    game.tick();
    game.renders_frame(vec![
        ".....11...",
        "..........",
        "..........",
        "..........",
        "..........",
        "....0000..",
    ]);
}

#[test]
fn holding_space_down_spawns_bullets_every_n_ticks() {
    let mut game = TestableGame::init()
        .with_dimensions(10, 6)
        .with_bullet_speed(2)
        .with_autofire_delay(2)
        .with_bullet_dimensions(2, 1)
        .build();

    game.key_is_down(KEY_SPACE);
    game.tick();
    game.renders_frame(vec![
        "..........",
        "..........",
        "..........",
        "..........",
        "....11....",
        "...0000...",
    ]);

    game.tick();
    game.renders_frame(vec![
        "..........",
        "..........",
        "....11....",
        "..........",
        "..........",
        "...0000...",
    ]);

    game.tick();
    game.renders_frame(vec![
        "....11....",
        "..........",
        "..........",
        "..........",
        "....11....",
        "...0000...",
    ]);
}

#[test]
fn releasing_space_resets_autofire_delay() {
    let mut game = TestableGame::init()
        .with_dimensions(10, 6)
        .with_bullet_speed(1)
        .with_autofire_delay(10)
        .with_bullet_dimensions(2, 1)
        .build();

    game.key_is_down(KEY_SPACE);
    game.tick();
    game.renders_frame(vec![
        "..........",
        "..........",
        "..........",
        "..........",
        "....11....",
        "...0000...",
    ]);
    game.key_is_up(KEY_SPACE);
    game.tick();
    game.key_is_down(KEY_SPACE);
    game.tick();
    game.renders_frame(vec![
        "..........",
        "..........",
        "....11....",
        "..........",
        "....11....",
        "...0000...",
    ]);
}