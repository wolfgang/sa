use crate::_tests::helpers::string_renderer::StringRenderer;
use crate::game::positioned::OnScreen;
use crate::game::renderer::GameRenderer;

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
    sr.register_sprite(SPRITE1, 2, 3);
    sr.register_sprite(SPRITE2, 3, 1);
    sr.draw_sprite(SPRITE1, 1, 2);
    sr.draw_sprite(SPRITE2, 1, 0);

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
fn clear_clears_frame() {
    let mut sr = StringRenderer::new(6, 3);
    sr.register_sprite(SPRITE2, 3, 1);
    sr.draw_sprite(SPRITE2, 1, 1);

    sr.assert_frame(vec![
        "......",
        ".222..",
        "......",
    ]);

    sr.clear();

    sr.assert_frame(vec![
        "......",
        "......",
        "......",
    ]);
}

#[test]
fn ignore_pixels_out_of_bounds() {
    let mut sr = StringRenderer::new(4, 4);
    sr.register_sprite(SPRITE1, 5, 6);
    sr.register_sprite(SPRITE2, 2, 2);
    sr.draw_sprite(SPRITE1, 1, 2);
    sr.draw_sprite(SPRITE2, -1, 0);

    sr.assert_frame(vec![
        "2...",
        "2...",
        ".111",
        ".111"
    ]);
}

struct TestObj {
    x: i32,
    y: i32,
}

impl OnScreen for TestObj {
    fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }
}

#[test]
fn draw_sprite_from_trait_object() {
    let mut sr = StringRenderer::new(4, 4);
    sr.register_sprite(SPRITE1, 2, 2);

    let obj = TestObj { x: 1, y: 1 };

    sr.draw_sprite_obj(SPRITE1, &obj);

    sr.assert_frame(vec![
        "....",
        ".11.",
        ".11.",
        "...."
    ]);
}

#[test]
fn sprite_log_returns_list_of_sprites_rendered() {
    let mut sr = StringRenderer::new(4, 4);
    sr.register_sprite(SPRITE1, 1, 2);
    sr.register_sprite(SPRITE2, 3, 4);

    sr.draw_sprite(SPRITE1, 0, 1);
    sr.draw_sprite(SPRITE2, 3, 1);
    sr.draw_sprite(SPRITE2, 4, 2);

    sr.assert_sprite_log_for(1, vec![
        "1, 0, 1"
    ]);

    sr.assert_sprite_log_for(2, vec![
        "2, 3, 1",
        "2, 4, 2"
    ])
}
