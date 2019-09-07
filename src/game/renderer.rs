use crate::game::positioned::Positioned;

pub trait GameRenderer {
    fn draw_sprite_obj<T: Positioned>(&mut self, id: u8, obj: &T) where Self: Sized;
    fn draw_sprite(&mut self, id: u8, x: i32, y: i32);
    fn clear(&mut self);
}
