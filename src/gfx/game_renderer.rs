pub trait GameRenderer {
    fn draw_sprite(&mut self, id: u8, x: i32, y: i32, width: u32, height: u32);
    fn clear(&mut self) {}
}
