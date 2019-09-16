use crate::game::game_object::GameObject;
use crate::game::geometry::{Rectanglef, Vector2f};
use crate::game::renderer::GameRenderer;

pub struct PlayerBullet {
    game_object: GameObject
}

impl PlayerBullet {
    pub fn new(rectangle: Rectanglef, speed: Vector2f) -> Self {
        let mut game_object = GameObject::new(1, rectangle, speed);
        game_object.set_move_direction(0, -1);
        Self { game_object }
    }

    pub fn tick(&mut self) {
        self.game_object.tick();
    }

    pub fn is_alive(&self) -> bool {
        let y = self.game_object.get_position().y;
        y >= 0.0
    }

    pub fn render<T>(&self, renderer: &mut T) where T: GameRenderer {
        renderer.draw_sprite_obj(&self.game_object)

    }
}
