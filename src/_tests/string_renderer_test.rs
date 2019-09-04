use std::borrow::Borrow;

use raylib::{Rectangle, Vector2};

trait GameRenderer {
    fn draw_sprite(position: Vector2, rectangle: Rectangle, id: u8);
}

type Frame = Vec<Vec<char>>;

struct StringRenderer {
    frame: Frame,
    width: usize,
    height: usize,
}

impl StringRenderer {
    pub fn new(width: usize, height: usize) -> Self {
        StringRenderer {
            frame: vec![vec!['.'; width]; height],
            width,
            height,
        }
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
fn assert_frame() {
    let sr = StringRenderer::new(3, 4);

    sr.assert_frame(vec![
        "...",
        "...",
        "...",
        "..."
    ])
}