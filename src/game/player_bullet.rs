use crate::game::game_object::GameObject;
use crate::game::geometry::{Rectanglef, Vector2f};
use crate::game::moving_sprite::MovingSprite;
use crate::game::player_ship::PlayerShip;
use crate::game::renderer::GameRenderer;

pub struct PlayerBullet {
    game_object: MovingSprite
}

impl PlayerBullet {
    pub fn new(rectangle: Rectanglef, speed: Vector2f) -> Self {
        let mut game_object = MovingSprite::new(1, rectangle, speed);
        game_object.set_move_direction(0, -1);
        Self { game_object }
    }
}

impl GameObject for PlayerBullet {
    fn tick(&mut self) {
        self.game_object.tick();
    }

    fn render(&self, renderer: &mut dyn GameRenderer) {
        renderer.draw_sprite_obj(&self.game_object)
    }

    fn is_alive(&self) -> bool {
        let y = self.game_object.get_position().y;
        y >= 0.0
    }
}
