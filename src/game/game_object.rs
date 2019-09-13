use crate::game::geometry::{Vector2, Vector2f};
use crate::game::positioned::OnScreen;
use crate::game::renderer::GameRenderer;

pub struct GameObject {
    sprite_id: u8,
    position: Vector2f,
    speed: Vector2f,
    move_direction: Vector2f,

}

impl GameObject {
    pub fn new(sprite_id: u8, x: f32, y: f32, speed: Vector2f) -> Self {
        Self {
            sprite_id,
            position: Vector2::with(x, y),
            speed,
            move_direction: Vector2::zero(),
        }
    }

    pub fn tick(&mut self) {
        self.position += self.move_direction * self.speed;
    }

    pub fn tick_and<F>(&mut self, and: F) where F: Fn(&mut Self) {
        self.tick();
        and(self);
    }

    pub fn render<T>(&self, renderer: &mut T) where T: GameRenderer {
        renderer.draw_sprite_obj(self.sprite_id, self)
    }

    pub fn set_move_direction(&mut self, dx: i32, dy: i32) {
        self.move_direction.x = dx as f32;
        self.move_direction.y = dy as f32;
    }

    pub fn get_position(&self) -> Vector2f {
        self.position
    }

    pub fn set_position(&mut self, x: f32, y: f32) {
        self.position.x = x;
        self.position.y = y;
    }
}

impl OnScreen for GameObject {
    fn position(&self) -> (i32, i32) {
        (self.position.x as i32, self.position.y as i32)
    }
}