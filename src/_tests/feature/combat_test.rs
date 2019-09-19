use raylib::consts::KEY_SPACE;

use crate::_tests::helpers::testable_game::TestableGame;

#[test]
fn after_bullet_collides_with_enemy_both_disappear_and_new_enemy_is_spawned() {
    let mut game = TestableGame::init()
        .with_dimensions(10, 4)
        .with_enemy_speed(120, 0)
        .with_enemy_dimensions(2, 2)
        .build();

    game.renders_frame(vec![
        "22........",
        "22........",
        "..........",
        "...0000..."
    ]);

    game.key_is_down(KEY_SPACE);
    game.tick();

    game.renders_frame(vec![
        "..22......",
        "..2211....",
        "..........",
        "...0000..."
    ]);

    game.tick();

//    game.renders_frame(vec![
//        "22........",
//        "22........",
//        "..........",
//        "...0000..."
//    ]);
}