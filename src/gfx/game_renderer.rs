pub trait GameRenderer {
    fn draw_sprite(&mut self, id: u8, x: u32, y: u32);
}

