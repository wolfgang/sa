use std::collections::HashMap;

use raylib::{Color, RaylibHandle, Rectangle, Texture2D, Vector2};

use crate::gfx::game_renderer::GameRenderer;

pub struct RaylibRenderer<'a> {
    rl: &'a RaylibHandle,
    sprite_sheet: Texture2D,
    sprites: HashMap<u8, Rectangle>,
}

impl<'a> RaylibRenderer<'a> {
    pub fn new(rl: &'a RaylibHandle) -> Self {
        let sprite_sheet = rl.load_texture("resources/spaceshooter_sheet.png");
        Self { rl, sprite_sheet, sprites: HashMap::with_capacity(10) }
    }

    pub fn register_sprite(&mut self, id: u8, source_rect: Rectangle) {
        self.sprites.insert(id, source_rect);
    }
}

impl GameRenderer for RaylibRenderer<'_> {
    fn draw_sprite(&mut self, id: u8, x: u32, y: u32, width: u32, height: u32) {
        let source_rec = self.sprites.get(&id).unwrap();
        let dest_rec = Rectangle { x: x as f32, y: y as f32, width: width as f32, height: height as f32 };
        self.rl.draw_texture_pro(&self.sprite_sheet, *source_rec, dest_rec, Vector2::zero(), 0.0, Color::WHITE);
    }
}