#[derive(Clone, PartialEq, Debug)]
pub struct RectangleU32 {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

impl RectangleU32 {
    pub fn intersect(rect1: &Self, rect2: &Self) -> bool {
        if rect1.x + (rect1.width as i32) <= rect2.x { return false }
        if rect1.x >= rect2.x + (rect2.width as i32) { return false }
        if rect1.y + (rect1.height as i32) <= rect2.y { return false }
        if rect1.y >= rect2.y + (rect2.height as i32) { return false }
        true
    }
}
