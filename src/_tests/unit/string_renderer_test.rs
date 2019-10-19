use crate::_tests::helpers::string_renderer::StringRenderer;
use crate::gfx::game_renderer::GameRenderer;

#[test]
fn frame_is_empty_after_construction() {
    let sr = StringRenderer::new(3, 4);
    sr.assert_frame(vec![
        "...",
        "...",
        "...",
        "..."
    ])
}

const SPRITE1: u8 = 1;
const SPRITE2: u8 = 2;

#[test]
fn draw_sprite_fills_rect_with_sprite_id() {
    let mut sr = StringRenderer::new(6, 6);
    sr.draw_sprite(SPRITE1, 1, 2, 2, 3);
    sr.draw_sprite(SPRITE2, 1, 0, 3, 1);

    sr.assert_frame(vec![
        ".222..",
        "......",
        ".11...",
        ".11...",
        ".11...",
        "......"
    ])
}

#[test]
fn clear() {
    let mut sr = StringRenderer::new(3, 2);
    sr.draw_sprite(SPRITE1, 0, 0, 2, 2);
    sr.clear();
    sr.assert_frame(vec![
        "...",
        "..."]);
}

#[test]
fn ignore_pixels_out_of_bounds() {
    let mut sr = StringRenderer::new(4, 4);
    sr.draw_sprite(SPRITE1, 1, 2, 5, 6);
    sr.draw_sprite(SPRITE2, -1, 0, 2, 2);

    sr.assert_frame(vec![
        "2...",
        "2...",
        ".111",
        ".111"
    ]);
}

#[test]
fn sprite_log_returns_list_of_sprites_rendered() {
    let mut sr = StringRenderer::new(4, 4);

    sr.draw_sprite(SPRITE1, 0, 1, 1, 2);
    sr.draw_sprite(SPRITE2, 3, 1, 3, 4);
    sr.draw_sprite(SPRITE2, 4, 2, 3, 4);

    sr.assert_sprite_log_for(1, vec![
        "1, 0, 1"
    ]);

    sr.assert_sprite_log_for(2, vec![
        "2, 3, 1",
        "2, 4, 2"
    ])
}


