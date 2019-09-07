use crate::game::positioned::Positioned;

pub trait GameRenderer {
    fn draw_sprite_obj(&mut self, id: u8, obj: &dyn Positioned) {
        let (x, y) = obj.position();
        self.draw_sprite(id, x, y)
    }

    fn draw_sprite(&mut self, id: u8, x: i32, y: i32);
    fn clear(&mut self);
}
