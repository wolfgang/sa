use std::collections::HashMap;
use std::rc::Rc;

use raylib::{Color, RaylibHandle, Rectangle, Texture2D, Vector2};

use crate::game::renderer::GameRenderer;

pub struct RaylibRenderer {
    rl: Rc<RaylibHandle>,
    sprite_sheet: Texture2D,
    sprites: HashMap<u8, Rectangle>
}

impl RaylibRenderer {
    pub fn new(rl: Rc<RaylibHandle>) -> Self {
        let sprite_sheet = rl.load_texture("resources/spaceshooter_sheet.png");
        Self {
            rl,
            sprite_sheet,
            sprites: HashMap::new()
        }
    }

    pub fn register_sprite(&mut self, id: u8, source_rec: Rectangle) {
        self.sprites.insert(id, source_rec);
    }
}

impl GameRenderer for RaylibRenderer {
    fn draw_sprite(&mut self, id: u8, x: i32, y: i32) {
        let source_rec = self.sprites.get(&id).unwrap();

        let position = Vector2 { x: x as f32, y: y as f32 };
        self.rl.draw_texture_rec(&self.sprite_sheet, *source_rec, position, Color::WHITE)
    }

    fn clear(&mut self) {
        self.rl.clear_background(Color::BLACK);
    }
}