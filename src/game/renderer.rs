use crate::game::positioned::OnScreen;

pub trait GameRenderer {
    fn draw_sprite_obj_old<T>(&mut self, id: u8, obj: &T) where T: OnScreen {
        let (x, y) = obj.position();
        self.draw_sprite(id, x, y)
    }

    fn draw_sprite(&mut self, id: u8, x: i32, y: i32);
    fn clear(&mut self);
}
