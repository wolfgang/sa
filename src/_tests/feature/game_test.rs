use raylib::consts::KEY_LEFT;

use crate::_tests::helpers::input_stub::InputStub;
use crate::_tests::helpers::string_renderer::StringRenderer;
use crate::game::game::Game;

#[test]
fn render_player_ship_at_bottom_of_screen_centered() {
    let game = Game::init()
        .with_dimensions(10, 5)
        .with_ship_dimensions(4, 1)
        .build();

    let mut renderer = StringRenderer::new(10, 5);

    renderer.register_sprite(0, 4, 1);

    game.render(&mut renderer);

    renderer.assert_frame(vec![
        "..........",
        "..........",
        "..........",
        "..........",
        "...0000...",
    ])

}

#[test]
fn ship_movement() {
    let input = InputStub::new_rc();
    let mut game = Game::init()
        .with_input(input.clone())
        .with_dimensions(10, 3)
        .with_ship_dimensions(4, 1)
        .with_fps(60)
        .with_ship_speed(2)
        .build();

    let mut renderer = StringRenderer::new(10, 3);

    renderer.register_sprite(0, 4, 1);

    game.render(&mut renderer);
    renderer.assert_frame(vec![
        "..........",
        "..........",
        "...0000...",
    ]);

    input.borrow_mut().key_is_down(KEY_LEFT);
    game.tick();

    game.render(&mut renderer);
    renderer.assert_frame(vec![
        "..........",
        "..........",
        ".0000.....",
    ]);
}