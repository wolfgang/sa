use crate::gfx::sprite::Sprite;

pub trait GameRenderer {
    fn draw_sprite_obj(&mut self, sprite: &dyn Sprite) {
        let (x, y) = sprite.position();
        self.draw_sprite(sprite.id(), x, y)
    }

    fn draw_sprite(&mut self, id: u8, x: i32, y: i32);
    fn clear(&mut self);
}
