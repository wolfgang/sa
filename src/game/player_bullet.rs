use crate::game::game_object::GameObject;
use crate::game::renderer::GameRenderer;

pub struct PlayerBullet {
    game_object: GameObject
}

impl PlayerBullet {
    pub fn new(x: f32, y: f32, speed: u32) -> Self {
        let mut game_object = GameObject::new(1, x, y, (0, speed));
        game_object.set_move_direction(0, -1);
        Self { game_object }
    }

    pub fn tick(&mut self) {
        self.game_object.tick();
    }

    pub fn is_alive(&self) -> bool {
        let y = self.game_object.get_position().1;
        y >= 0
    }

    pub fn render<T>(&self, renderer: &mut T) where T: GameRenderer {
        self.game_object.render(renderer)
    }
}
