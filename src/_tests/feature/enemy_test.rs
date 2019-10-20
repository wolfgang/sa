use raylib::consts::KEY_SPACE;

use crate::_tests::helpers::testable_game::TestableGame;

#[test]
fn enemy_moves_from_left_to_right() {
    let mut game = TestableGame::init()
        .with_dimensions(10, 3)
        .with_enemy_count(1)
        .with_enemy_speed(4, 0)
        .build();

    game.tick();
    game.renders_frame(vec![
        "222.......",
        "..........",
        "...0000..."
    ]);

    game.tick();
    game.renders_frame(vec![
        "....222...",
        "..........",
        "...0000..."
    ]);

    game.tick();
    game.renders_frame(vec![
        ".......222",
        "..........",
        "...0000..."
    ]);

    game.tick();
    game.renders_frame(vec![
        "...222....",
        "..........",
        "...0000..."
    ]);

    game.tick();
    game.renders_frame(vec![
        "222.......",
        "..........",
        "...0000..."
    ]);

    game.tick();
    game.renders_frame(vec![
        "....222...",
        "..........",
        "...0000..."
    ]);

}

#[test]
fn after_enemy_leaves_screen_start_from_the_top() {
    let mut game = TestableGame::init()
        .with_dimensions(10, 3)
        .with_enemy_count(1)
        .with_enemy_speed(1, 2)
        .build();

    game.tick_times(2);
    game.renders_frame(vec![
        "..........",
        "..........",
        ".222000..."
    ]);

    game.tick();
    game.renders_frame(vec![
        "..222.....",
        "..........",
        "...0000..."
    ]);

    game.tick();
    game.renders_frame(vec![
        "..........",
        "..........",
        "...2220..."
    ]);
}

#[test]
fn if_enemy_is_hit_with_bullet_it_disappears_and_a_new_one_is_spawned() {
    let mut game = TestableGame::init()
        .with_dimensions(10, 6)
        .with_enemy_count(1)
        .with_enemy_speed(1, 0)
        .with_bullet_speed(2)
        .build();

    game.key_is_down(KEY_SPACE);
    game.tick();
    game.renders_frame(vec![
        "222.......",
        "..........",
        "..........",
        "..........",
        "....11....",
        "...0000..."
    ]);

    game.key_is_up(KEY_SPACE);
    game.tick();
    game.renders_frame(vec![
        ".222......",
        "..........",
        "....11....",
        "..........",
        "..........",
        "...0000..."
    ]);

    game.tick();
    game.renders_frame(vec![
        "..........",
        "..........",
        "..........",
        "..........",
        "..........",
        "...0000..."
    ]);

    game.tick();
    game.renders_frame(vec![
        "222.......",
        "..........",
        "..........",
        "..........",
        "..........",
        "...0000..."
    ]);

}