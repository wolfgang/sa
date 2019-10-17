use std::char;
use std::collections::HashMap;

use crate::gfx::game_renderer::GameRenderer;

type Frame = Vec<Vec<char>>;

pub struct StringRenderer {
    frame: Frame
}

impl GameRenderer for StringRenderer {
    fn draw_sprite(&mut self, id: u8, x: u32, y: u32, width: u32, height: u32) {
        let id_char = char::from_digit(id as u32, 10).unwrap();

        for row in 0..height {
            for column in 0..width {
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
            frame: Self::new_frame(width, height)
        }
    }

    fn new_frame(width: usize, height: usize) -> Frame {
        vec![vec!['.'; width]; height]
    }

    pub(crate) fn assert_frame(&self, expected_frame: Vec<&str>) {
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
