use crate::core::geometry::{Rectanglef, Vector2f};
use crate::game::game_objects_manager::GameObjectsManagerRef;
use crate::gfx::renderer::GameRenderer;

use super::game_object::GameObject;
use super::moving_sprite::MovingSprite;

pub struct PlayerBullet {
    moving_sprite: MovingSprite,
    game_objects_manager: GameObjectsManagerRef
}

impl PlayerBullet {
    pub fn new(rectangle: Rectanglef, speed: Vector2f, game_objects_manager: GameObjectsManagerRef) -> Self {
        let mut moving_sprite = MovingSprite::new(1, rectangle, speed);
        moving_sprite.set_move_direction(0, -1);
        Self { moving_sprite, game_objects_manager }
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
        y >= 0.0
    }
}
