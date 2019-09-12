use crate::_tests::helpers::testable_game::TestableGame;

#[test]
fn enemy_goes_from_left_to_right() {
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

    game.tick();

    game.renders_frame(vec![
        "..22......",
        "..22......",
        "..........",
        "...0000..."
    ]);

    game.tick_times(3);
    game.renders_frame(vec![
        "........22",
        "........22",
        "..........",
        "...0000..."
    ]);

    game.tick();
    game.renders_frame(vec![
        "......22..",
        "......22..",
        "..........",
        "...0000..."
    ]);

    game.tick_times(3);
    game.renders_frame(vec![
        "22........",
        "22........",
        "..........",
        "...0000..."
    ]);

    game.tick();
    game.renders_frame(vec![
        "..22......",
        "..22......",
        "..........",
        "...0000..."
    ]);

}