use raylib::consts::{KEY_LEFT, KEY_RIGHT};

use crate::_tests::helpers::testable_game::TestableGame;

#[test]
fn render_player_ship_at_bottom_of_screen_centered() {
    let mut game = TestableGame::init()
        .with_dimensions(10, 5)
        .build();

    game.render();
    game.assert_frame(vec![
        "..........",
        "..........",
        "..........",
        "..........",
        "...0000...",
    ])

}

#[test]
fn ship_movement() {
    let mut game = TestableGame::init()
        .with_dimensions(10, 2)
        .with_fps(60)
        .with_ship_speed(120)
        .build();

    game.render();
    game.assert_frame(vec![
        "..........",
        "...0000...",
    ]);

    game.key_is_down(KEY_LEFT);
    game.tick();

    game.render();
    game.assert_frame(vec![
        "..........",
        ".0000.....",
    ]);


    game.key_is_up(KEY_LEFT);
    game.key_is_down(KEY_RIGHT);
    game.tick();
    game.tick();

    game.render();
    game.assert_frame(vec![
        "..........",
        ".....0000.",
    ]);

    game.key_is_up(KEY_RIGHT);
    game.tick();
    game.tick();

    game.render();
    game.assert_frame(vec![
        "..........",
        ".....0000.",
    ]);


}