use crate::game::geometry::{Rectanglef, Vector2, Vector2f};
use crate::game::positioned::OnScreen;
use crate::game::renderer::GameRenderer;

pub struct GameObject {
    sprite_id: u8,
    speed: Vector2f,
    move_direction: Vector2f,
    rectangle: Rectanglef,
}

impl GameObject {
    pub fn new(sprite_id: u8, rectangle: Rectanglef, speed: Vector2f) -> Self {
        Self {
            sprite_id,
            speed,
            rectangle,
            move_direction: Vector2::zero(),
        }
    }

    pub fn tick(&mut self) {
        self.rectangle.position += self.move_direction * self.speed;
    }

    pub fn render<T>(&self, renderer: &mut T) where T: GameRenderer {
        renderer.draw_sprite_obj(self.sprite_id, self)
    }

    pub fn set_move_direction(&mut self, dx: i32, dy: i32) {
        self.move_direction.x = dx as f32;
        self.move_direction.y = dy as f32;
    }

    pub fn mult_move_direction(&mut self, dx: i32, dy: i32) {
        self.move_direction.x *= dx as f32;
        self.move_direction.y *= dy as f32;
    }

    pub fn get_position(&self) -> Vector2f {
        self.rectangle.position
    }

    pub fn set_position(&mut self, x: f32, y: f32) {
        self.rectangle.position.x = x;
        self.rectangle.position.y = y;
    }

    pub fn is_inside_width(&self, width: f32) -> bool {
        let x = self.rectangle.position.x;
        x >= 0.0 && x <= width - self.rectangle.width
    }

    pub fn is_inside_of(&self, rect: &Rectanglef) -> bool {
        self.rectangle.is_inside_of(rect)
    }

    pub fn snap_to_width(&mut self, width: f32) {
        let x = self.rectangle.position.x;
        self.rectangle.position.x = f32::max(0.0, f32::min(x, width - self.rectangle.width));
    }
}

impl OnScreen for GameObject {
    fn position(&self) -> (i32, i32) {
        (self.rectangle.position.x as i32, self.rectangle.position.y as i32)
    }
}