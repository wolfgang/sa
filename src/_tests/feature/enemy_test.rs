use crate::_tests::helpers::testable_game::TestableGame;

#[ignore]
#[test]
fn enemy_starts_on_top_left_of_screen() {
    let mut game = TestableGame::init()
        .with_dimensions(10, 5)
        .build();

    game.renders_frame(vec![
        "22........",
        "22........",
        "..........",
        "..........",
        "...0000..."
    ])
}