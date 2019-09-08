use raylib::Rectangle;

pub struct SpriteRegistry {}

impl SpriteRegistry {
    pub fn from_xml(xml: String) -> Self {
        Self {}
    }

    pub fn get_source_rec(&self, name: &str) -> Rectangle {
        Rectangle {
            x: 0.0,
            y: 0.0,
            width: 0.0,
            height: 0.0
        }
    }
}