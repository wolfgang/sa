use crate::game::renderer::GameRenderer;

pub struct PlayerBullet {
    x: i32,
    y: i32,
}

impl PlayerBullet {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn render(&self, renderer: &mut dyn GameRenderer) {
        renderer.draw_sprite(1, self.x, self.y);
    }
}
