use crate::core::geometry::{Rectanglef, Vector2f};
use crate::game::objects::game_object::GameObjectRef;
use crate::gfx::renderer::GameRenderer;

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

    fn render(&self, renderer: &mut dyn GameRenderer) {
        self.moving_sprite.render(renderer);
    }

    fn is_alive(&self) -> bool {
        let y = self.moving_sprite.get_position().y;
        self.is_alive && y >= 0.0
    }

    fn check_collisions(&mut self, targets: &Vec<GameObjectRef>) {
        for t in targets.iter() {
            if t.borrow_mut().process_collision(&self.moving_sprite) {
                self.is_alive = false;
            }
        }
    }

}
