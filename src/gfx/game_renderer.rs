pub trait GameRenderer {
    fn draw_sprite(&mut self, id: u8, x: u32, y: u32, width: u32, height: u32);
}
