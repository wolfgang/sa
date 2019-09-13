use crate::game::geometry::{Vector2, Vector2f};
use crate::game::positioned::Positioned;
use crate::game::renderer::GameRenderer;

pub struct GameObject {
    sprite_id: u8,
    position: Vector2f,
    speed: Vector2f,
    move_direction: Vector2<i32>,

}

impl GameObject {
    pub fn new(sprite_id: u8, x: f32, y: f32, speed: (u32, u32)) -> Self {
        Self {
            sprite_id,
            position: Vector2::new(x, y),
            speed: Vector2::new(speed.0 as f32, speed.1 as f32),
            move_direction: Vector2::new(0, 0),
        }
    }

    pub fn tick(&mut self) {
        self.position.x += self.move_direction.x as f32 * self.speed.x;
        self.position.y += self.move_direction.y as f32 * self.speed.y;
    }


    pub fn tick_and<F>(&mut self, and: F) where F: Fn(&mut Self) {
        self.tick();
        and(self);
    }

    pub fn render<T>(&self, renderer: &mut T) where T: GameRenderer {
        renderer.draw_sprite_obj(self.sprite_id, self)
    }

    pub fn set_move_direction(&mut self, dx: i32, dy: i32) {
        self.move_direction.x = dx;
        self.move_direction.y = dy;
    }

    pub fn get_position(&self) -> Vector2f {
        self.position
    }

    pub fn set_position(&mut self, x: i32, y: i32) {
        self.position.x = x as f32;
        self.position.y = y as f32;
    }
}

impl Positioned for GameObject {
    fn position(&self) -> (i32, i32) {
        (self.position.x as i32, self.position.y as i32)
    }
}