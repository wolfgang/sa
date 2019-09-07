use crate::game::positioned::Positioned;
use crate::game::renderer::GameRenderer;

const BULLET_SPEED: u32 = 60;

pub struct PlayerBullet {
    x: i32,
    y: i32,
}

impl PlayerBullet {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn tick(&mut self) {
        self.y = (self.y as f32 - BULLET_SPEED as f32 / 60.0) as i32
    }

    pub fn render(&self, renderer: &mut dyn GameRenderer) {
        renderer.draw_sprite_obj(1, self);
    }
}

impl Positioned for PlayerBullet {
    fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }
}