use raylib::consts::KEY_SPACE;

use crate::_tests::helpers::testable_game::TestableGame;

const BULLET_SPRITE: u8 = 1;

#[test]
fn pressing_space_spawn_bullet_in_front_of_player_ship() {
    let mut game = TestableGame::init()
        .with_dimensions(10, 5)
        .build();

    game.register_sprite(BULLET_SPRITE, 2, 2);

    game.key_is_down(KEY_SPACE);
    game.tick();

    game.renders_frame(vec![
        "..........",
        "..........",
        "....11....",
        "....11....",
        "...0000...",
    ])
}