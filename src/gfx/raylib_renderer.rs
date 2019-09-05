use std::rc::Rc;

use raylib::{Color, RaylibHandle, Rectangle, Texture2D, Vector2};

use crate::gfx::game_renderer::GameRenderer;

pub struct RaylibRenderer {
    rl: Rc<RaylibHandle>,
    sprite_sheet: Texture2D,
}

impl RaylibRenderer {
    pub fn new(rl: Rc<RaylibHandle>) -> Self {
        let sprite_sheet = rl.load_texture("resources/spaceshooter_sheet.png");
        Self {
            rl,
            sprite_sheet,
        }
    }
}

impl GameRenderer for RaylibRenderer {
    fn draw_sprite(&mut self, _id: u8, x: u32, y: u32) {
        let source_rec = Rectangle {
            x: 325.0,
            y: 0.0,
            width: 98.0,
            height: 75.0,
        };

        let position = Vector2 { x: x as f32, y: y as f32 };
        self.rl.draw_texture_rec(&self.sprite_sheet, source_rec, position, Color::WHITE)
    }
}