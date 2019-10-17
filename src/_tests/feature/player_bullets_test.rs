use raylib::consts::{KEY_RIGHT, KEY_SPACE};

use crate::_tests::helpers::testable_game::TestableGame;

#[test]
fn pressing_space_spawns_bullet_from_ship_center() {
    let mut game = TestableGame::init()
        .with_dimensions(10, 8)
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
        "..........",
        "..........",
        ".....11...",
        "..........",
        "..........",
        "....0000..",
    ]);

}
