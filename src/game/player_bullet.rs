use crate::game::positioned::Positioned;
use crate::game::renderer::GameRenderer;

pub struct PlayerBullet {
    x: i32,
    y: i32,
    fps: f32,
    speed: f32
}

impl PlayerBullet {
    pub fn new(x: i32, y: i32, fps: u32, speed: u32) -> Self {
        Self { x, y, fps: fps as f32, speed: speed as f32 }
    }

    pub fn tick(&mut self) {
        self.y = (self.y as f32 - self.speed) as i32
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