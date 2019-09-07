use crate::game::positioned::Positioned;
use crate::game::renderer::GameRenderer;

#[derive(Clone)]
pub struct PlayerBullet {
    x: i32,
    y: i32,
    speed: u32
}

impl PlayerBullet {
    pub fn new(x: i32, y: i32, speed: u32) -> Self {
        Self { x, y, speed }
    }

    pub fn tick(&mut self) -> bool {
        self.y -= self.speed as i32;
        self.y >= 0
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