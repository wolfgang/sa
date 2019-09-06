use crate::game::renderer::GameRenderer;

pub struct PlayerShip {
    pub width: i32,
    pub height: i32,
}

impl PlayerShip {
    pub fn render(&self, renderer: &mut dyn GameRenderer) {
        renderer.draw_sprite(0, 0, 0)
    }
}