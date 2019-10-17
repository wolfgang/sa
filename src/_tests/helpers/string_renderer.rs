use std::char;

use crate::gfx::game_renderer::GameRenderer;

type Frame = Vec<Vec<char>>;

pub struct StringRenderer {
    frame: Frame,
    width: usize,
    height: usize,
}

impl GameRenderer for StringRenderer {
    fn draw_sprite(&mut self, id: u8, x: i32, y: i32, width: u32, height: u32) {
        let id_char = char::from_digit(id as u32, 10).unwrap();

        for row in 0..height {
            for column in 0..width {
                let pixel_x = column as i32 + x;
                let pixel_y = row as i32 + y;
                if pixel_x >= 0 && pixel_x < self.width as i32 &&
                    pixel_y >= 0 && pixel_y < self.height as i32 {
                    self.frame[pixel_y as usize][pixel_x as usize] = id_char;
                }
            }
        }
    }

    fn clear(&mut self) {
        self.frame = Self::new_frame(self.width, self.height);
    }
}

impl StringRenderer {
    pub fn new(width: usize, height: usize) -> Self {
        StringRenderer {
            width,
            height,
            frame: Self::new_frame(width, height),
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
