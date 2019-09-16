use crate::game::geometry::{Rectanglef, Vector2, Vector2f};
use crate::game::renderer::GameRenderer;
use crate::game::sprite::Sprite;

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
        renderer.draw_sprite_obj(self)
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

    pub fn is_inside_of(&self, rect: &Rectanglef) -> bool {
        self.rectangle.is_inside_of(rect)
    }

    pub fn snap_to(&mut self, rec: &Rectanglef) {
        let x = self.rectangle.position.x;
        let y = self.rectangle.position.y;
        self.rectangle.position.x = Self::clamp(x, 0.0, rec.width - self.rectangle.width);
        self.rectangle.position.y = Self::clamp(y, 0.0, rec.height - self.rectangle.height);
    }

    fn clamp(val: f32, min: f32, max: f32) -> f32 {
        f32::max(min, f32::min(val, max))
    }
}

impl Sprite for GameObject {
    fn position(&self) -> (i32, i32) {
        (self.rectangle.position.x as i32, self.rectangle.position.y as i32)
    }

    fn id(&self) -> u8 {
        self.sprite_id
    }
}