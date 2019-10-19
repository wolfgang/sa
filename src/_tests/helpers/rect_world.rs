use std::collections::HashMap;

use crate::core::rectangle::RectangleU32;

pub struct RectWorld {
    rectangles: HashMap<u8, RectangleU32>
}

impl RectWorld {
    pub fn new(data: Vec<&str>) -> Self {
        let mut obj = Self { rectangles: HashMap::with_capacity(10) };
        obj.parse_rects(data);
        obj
    }

    pub fn rectangle(&self, id: u8) -> RectangleU32 {
        self.rectangles.get(&id).unwrap().clone()
    }

    fn parse_rects(&mut self, data: Vec<&str>) {
        for (y, line) in data.iter().enumerate() {
            for (x, point) in line.chars().enumerate() {
                if point.is_digit(10) {
                    let id = point.to_digit(10).unwrap() as u8;
                    self.process_rect(id, x, y);
                }
            }
        }
    }


    fn process_rect(&mut self, id: u8, x: usize, y: usize) -> () {
        if !self.rectangles.contains_key(&id) {
            self.rectangles.insert(id, RectangleU32 { x: x as i32, y: y as i32, width: 0, height: 0 });
        } else {
            let rect = self.rectangles.get_mut(&id).unwrap();
            rect.width = 1 + (x as i32 - rect.x).abs() as u32;
            rect.height = 1 + (y as i32 - rect.y).abs() as u32;
        }
    }
}
