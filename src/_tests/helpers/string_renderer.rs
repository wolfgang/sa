use std::char;
use std::collections::HashMap;

use crate::game::renderer::GameRenderer;

type Frame = Vec<Vec<char>>;
type SpriteLogEntry = (u8, i32, i32);

pub struct StringRenderer {
    width: usize,
    height: usize,
    frame: Frame,
    sprites: HashMap<u8, (u8, u8)>,
    sprite_log: Vec<SpriteLogEntry>,
}

impl GameRenderer for StringRenderer {
    fn draw_sprite(&mut self, id: u8, x: i32, y: i32) {
        let (width, height) = self.sprites.get(&id).unwrap();
        let id_char = char::from_digit(id as u32, 10).unwrap();

        self.sprite_log.push((id, x, y));

        for row in 0..*height {
            for column in 0..*width {
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
        self.frame = Self::new_frame(self.width, self.height)
    }
}

impl StringRenderer {
    pub fn new(width: usize, height: usize) -> Self {
        StringRenderer {
            width,
            height,
            frame: Self::new_frame(width, height),
            sprites: HashMap::with_capacity(10),
            sprite_log: Vec::with_capacity(10),
        }
    }

    pub fn register_sprite(&mut self, id: u8, width: u8, height: u8) {
        self.sprites.insert(id, (width, height));
    }

    pub(crate) fn assert_frame(&self, expected_frame: Vec<&str>) {
        let max_checked_lines = self.frame.len();
        assert_eq!(
            self.frame_as_strings()[0..max_checked_lines].join("\n"),
            expected_frame.join("\n")
        )
    }

    pub fn assert_sprite_log_for(&self, wanted_id: u8, expected_log: Vec<&str>) {
        let log_as_string: Vec<String> = self.sprite_log
            .iter()
            .filter(|(id, _, _)| *id == wanted_id)
            .map(|(id, x, y)| { format!("{}, {}, {}", id, x, y) })
            .collect();

        assert_eq!(log_as_string, expected_log);
    }

    fn new_frame(width: usize, height: usize) -> Frame {
        vec![vec!['.'; width]; height]
    }

    fn frame_as_strings(&self) -> Vec<String> {
        self.frame
            .iter()
            .map(|row| { row.iter().collect() })
            .collect()
    }
}
