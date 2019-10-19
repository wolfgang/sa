use crate::_tests::helpers::testable_game::TestableGame;

#[test]
fn enemy_moves_from_left_to_right() {
    let mut game = TestableGame::init()
        .with_dimensions(10, 5)
        .with_enemy_count(1)
        .build();

    game.renders_frame(vec![
        "222.......",
        "..........",
        "..........",
        "..........",
        "...0000..."
    ]);
}