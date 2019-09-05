use crate::_tests::helpers::string_renderer::StringRenderer;
use crate::game::game::Game;

#[test]
fn render_player_ship_at_bottom_of_screen_centered() {
    let game = Game::init()
        .with_dimensions(9, 5)
        .with_ship_dimensions(3, 1)
        .build();

    let mut renderer = StringRenderer::new(9, 5);

    renderer.register_sprite(0, 1, 3);

    game.render(&mut renderer);

    renderer.assert_frame(vec![
        ".........",
        ".........",
        ".........",
        ".........",
        "...000...",
    ])

}