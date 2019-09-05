use std::char;
use std::collections::HashMap;

use crate::gfx::game_renderer::GameRenderer;

type Frame = Vec<Vec<char>>;
struct StringRenderer {
    frame: Frame,
    sprites: HashMap<u8, (u8, u8)>

}

impl GameRenderer for StringRenderer {
    fn draw_sprite(&mut self, id: u8, x: u32, y: u32) {
        let (width, height) = self.sprites.get(&id).unwrap();
        let id_char = char::from_digit(id as u32, 10).unwrap();

        for row in 0..*height {
            for column in 0..*width {
                let pixel_x = (column as u32 + x) as usize;
                let pixel_y = (row as u32 + y) as usize;
                self.frame[pixel_y][pixel_x] = id_char;
            }
        }
    }
}

impl StringRenderer {
    pub fn new(width: usize, height: usize) -> Self {
        StringRenderer {
            frame: Self::new_frame(width, height),
            sprites: HashMap::with_capacity(10)
        }
    }

    pub fn register_sprite(&mut self, id: u8, width: u8, height: u8) {
        self.sprites.insert(id, (width, height));
    }

    fn new_frame(width: usize, height: usize) -> Frame {
        vec![vec!['.'; width]; height]
    }

    fn assert_frame(&self, expected_frame: Vec<&str>) {
        let max_checked_lines = self.frame.len();
        assert_eq!(
            self.frame_as_strings()[0..max_checked_lines].join("\n"),
            expected_frame.join("\n")
        )
    }

    fn frame_as_strings(&self) -> Vec<String> {
        self.frame
            .iter()
            .map(|row| { row.iter().collect() })
            .collect()
    }
}


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
