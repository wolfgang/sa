use std::collections::HashMap;

struct RectWorld {
    rectangles: HashMap<u8, Rectangle>
}

impl RectWorld {
    pub fn new(data: Vec<&str>) -> Self {
        let mut obj = Self { rectangles: HashMap::with_capacity(10) };
        obj.parse_rects(data);
        obj
    }

    pub fn rectangle(&self, id: u8) -> Rectangle {
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
            self.rectangles.insert(id, Rectangle { x: x as u32, y: y as u32, width: 0, height: 0 });
        } else {
            let rect = self.rectangles.get_mut(&id).unwrap();
            rect.width = x as u32 - rect.x + 1;
            rect.height = y as u32 - rect.y + 1;
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
struct Rectangle {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

#[test]
fn get_rectangles() {
    let rects = RectWorld::new(vec![
        ".1----|...",
        ".|..2-|-|.",
        ".|..|.|.|.",
        ".|----1.|.",
        "....|...|.",
        "....|---2.",
    ]);

    assert_eq!(rects.rectangle(1), Rectangle { x: 1, y: 0, width: 6, height: 4 });
    assert_eq!(rects.rectangle(2), Rectangle { x: 4, y: 1, width: 5, height: 5 });
}
