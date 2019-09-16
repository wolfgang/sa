use crate::game::game_object::GameObject;
use crate::game::geometry::{Rectanglef, Vector2f};
use crate::game::moving_sprite::MovingSprite;
use crate::game::renderer::GameRenderer;

pub struct PlayerBullet {
    moving_sprite: MovingSprite
}

impl PlayerBullet {
    pub fn new(rectangle: Rectanglef, speed: Vector2f) -> Self {
        let mut moving_sprite = MovingSprite::new(1, rectangle, speed);
        moving_sprite.set_move_direction(0, -1);
        Self { moving_sprite }
    }
}

impl GameObject for PlayerBullet {
    fn tick(&mut self) {
        self.moving_sprite.tick();
    }

    fn render(&self, renderer: &mut dyn GameRenderer) {
        renderer.draw_sprite_obj(&self.moving_sprite)
    }

    fn is_alive(&self) -> bool {
        let y = self.moving_sprite.get_position().y;
        y >= 0.0
    }
}
