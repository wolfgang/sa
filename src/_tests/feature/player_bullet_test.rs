use raylib::consts::*;

use crate::_tests::helpers::testable_game::TestableGame;

#[test]
fn pressing_space_spawns_bullet_which_flies_up_1_pixel_per_frame() {
    let mut game = make_game();

    game.key_is_down(KEY_RIGHT);
    game.tick();

    game.key_is_up(KEY_RIGHT);
    game.key_is_down(KEY_SPACE);

    game.tick();
    game.renders_frame(vec![
        "..........",
        "..........",
        "......11..",
        "..........",
        ".....0000.",
    ]);

    game.tick();
    game.renders_frame(vec![
        "..........",
        "......11..",
        "..........",
        "..........",
        ".....0000.",
    ]);


}


#[test]
fn wait_half_second_before_spawning_another_bullet_when_space_is_down() {
    let mut game = make_game();

    game.key_is_down(KEY_SPACE);
    game.tick();
    game.renders_frame(vec![
        "..........",
        "..........",
        "....11....",
        "..........",
        "...0000...",
    ]);

    game.key_is_down(KEY_LEFT);
    game.tick_times(29);

    game.renders_frame(vec![
        "..........",
        "..........",
        "..........",
        "..........",
        "0000......",
    ]);

    game.key_is_up(KEY_LEFT);
    game.tick_times(1);

    game.renders_frame(vec![
        "..........",
        "..........",
        ".11.......",
        "..........",
        "0000......",
    ])
}

#[test]
fn releasing_space_makes_next_bullet_spawn_immediately() {
    let mut game = make_game();

    game.key_is_down(KEY_SPACE);
    game.tick();
    game.renders_frame(vec![
        "..........",
        "..........",
        "....11....",
        "..........",
        "...0000...",
    ]);

    game.key_is_up(KEY_SPACE);
    game.tick();
    game.key_is_down(KEY_SPACE);
    game.tick();

    game.renders_frame(vec![
        "....11....",
        "..........",
        "....11....",
        "..........",
        "...0000...",
    ]);
}

#[test]
fn bullet_is_removed_after_leaving_screen() {
    let mut game = make_game();

    game.key_is_down(KEY_SPACE);
    game.tick();
    game.renders_frame(vec![
        "..........",
        "..........",
        "....11....",
        "..........",
        "...0000...",
    ]);

    game.key_is_up(KEY_SPACE);
    game.loop_times(10);

    game.assert_sprite_log_for(1, vec![
        "1, 4, 2",
        "1, 4, 1",
        "1, 4, 0",
    ])
}


fn make_game() -> TestableGame {
    TestableGame::init()
        .with_dimensions(10, 5)
        .build()
}