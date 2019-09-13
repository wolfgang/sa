use crate::game::geometry::Vector2;
use crate::game::positioned::Positioned;
use crate::game::renderer::GameRenderer;

pub struct GameObject {
    sprite_id: u8,
    position: Vector2<i32>,
    speed: (u32, u32),
    move_direction: (i32, i32),

}

impl GameObject {
    pub fn new(sprite_id: u8, x: i32, y: i32, speed: (u32, u32)) -> Self {
        Self {
            sprite_id,
            position: Vector2::new(x, y),
            speed,
            move_direction: (0, 0),
        }
    }

    pub fn tick(&mut self) {
        self.position.x += self.move_direction.0 * self.speed.0 as i32;
        self.position.y += self.move_direction.1 * self.speed.1 as i32;
    }


    pub fn tick_and<F>(&mut self, and: F) where F: Fn(&mut Self) {
        self.tick();
        and(self);
    }

    pub fn render<T>(&self, renderer: &mut T) where T: GameRenderer {
        renderer.draw_sprite_obj(self.sprite_id, self)
    }

    pub fn set_move_direction(&mut self, dx: i32, dy: i32) {
        self.move_direction = (dx, dy);
    }

    pub fn get_position(&self) -> (i32, i32) {
        (self.position.x, self.position.y)
    }

    pub fn set_position(&mut self, x: i32, y: i32) {
        self.position.x = x;
        self.position.y = y;
    }
}

impl Positioned for GameObject {
    fn position(&self) -> (i32, i32) {
        (self.position.x, self.position.y)
    }
}