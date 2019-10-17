use raylib::consts::*;

use crate::_tests::helpers::input_stub::InputStub;
use crate::_tests::helpers::string_renderer::StringRenderer;
use crate::_tests::helpers::testable_game::TestableGame;
use crate::game::game::Game;

#[test]
fn render_player_ship_at_bottom_of_screen_centered() {
    let mut game = TestableGame::init()
        .with_dimensions(10, 5)
        .build();

    game.renders_frame(vec![
        "..........",
        "..........",
        "..........",
        "..........",
        "...0000..."
    ])
}

#[test]
fn move_ship_left_right_according_to_input() {
    let mut game = TestableGame::init()
        .with_dimensions(10, 2)
        .with_ship_speed(1)
        .build();

    let mut renderer = StringRenderer::new(10, 2);

    game.renders_frame(vec![
        "..........",
        "...0000..."
    ]);

    game.key_is_down(KEY_LEFT);
    game.tick();
    game.renders_frame(vec![
        "..........",
        "..0000...."
    ]);

    game.key_is_up(KEY_LEFT);

    game.tick();
    game.renders_frame(vec![
        "..........",
        "..0000...."
    ]);

    game.key_is_down(KEY_RIGHT);
    game.tick();
    game.renders_frame(vec![
        "..........",
        "...0000..."
    ]);
}