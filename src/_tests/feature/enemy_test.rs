use crate::_tests::helpers::testable_game::TestableGame;

#[test]
fn enemy_starts_on_top_left_of_screen() {
    let mut game = TestableGame::init()
        .with_dimensions(10, 5)
        .with_enemy_speed(60, 30)
        .build();

    game.renders_frame(vec![
        "22........",
        "22........",
        "..........",
        "..........",
        "...0000..."
    ]);

    game.tick();

    game.renders_frame(vec![
        ".22.......",
        ".22.......",
        "..........",
        "..........",
        "...0000..."
    ]);

}