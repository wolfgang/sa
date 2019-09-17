use crate::core::geometry::{Rectanglef, Vector2f};

use super::game_object::GameObject;
use super::moving_sprite::MovingSprite;

pub struct PlayerBullet {
    moving_sprite: MovingSprite,
    is_alive: bool
}

impl PlayerBullet {
    pub fn new(rectangle: Rectanglef, speed: Vector2f) -> Self {
        let mut moving_sprite = MovingSprite::new(1, rectangle, speed);
        moving_sprite.set_move_direction(0, -1);
        Self { moving_sprite, is_alive: true }
    }
}

impl GameObject for PlayerBullet {
    fn tick(&mut self) {
        self.moving_sprite.tick();
    }

    fn collider(&self) -> &MovingSprite {
        &self.moving_sprite
    }


    fn is_alive(&self) -> bool {
        let y = self.moving_sprite.get_position().y;
        self.is_alive && y >= 0.0
    }

    fn on_collision(&mut self) {
        self.is_alive = false;
    }


}
