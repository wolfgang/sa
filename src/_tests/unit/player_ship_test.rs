use crate::_tests::helpers::string_renderer::StringRenderer;
use crate::game::player_ship::PlayerShip;

#[test]
fn renders_sprite_with_id_0() {
    let mut renderer = StringRenderer::new(10, 1);
    renderer.register_sprite(0, 4, 1);

    let ship = PlayerShip {
        width: 4,
        height: 1,
    };

    ship.render(&mut renderer);

    renderer.assert_frame(vec!["0000......"])
}