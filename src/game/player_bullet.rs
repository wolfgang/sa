use crate::game::positioned::Positioned;
use crate::game::renderer::GameRenderer;

#[derive(Clone)]
pub struct PlayerBullet {
    x: i32,
    y: i32,
    speed: u32,
    move_direction: i32,
}

impl PlayerBullet {
    pub fn new(x: i32, y: i32, speed: u32) -> Self {
        Self { x, y, speed, move_direction: -1 }
    }

    pub fn tick(&mut self) {
        self.y += self.move_direction * self.speed as i32;
    }

    pub fn is_alive(&self) -> bool {
        self.y >= 0
    }

    pub fn render<T>(&self, renderer: &mut T) where T: GameRenderer {
        renderer.draw_sprite_obj(1, self);
    }
}

impl Positioned for PlayerBullet {
    fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }
}