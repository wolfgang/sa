use crate::game::builder::GameBuilder;
use crate::game::game_object::GameObject;
use crate::game::geometry::{Rectanglef, Vector2};
use crate::game::renderer::GameRenderer;

pub struct EnemyShip {
    width: u32,
    max_x: u32,
    game_object: GameObject,
}

impl EnemyShip {
    pub fn from_game_builder(builder: &GameBuilder) -> Self {
        let rectangle = Rectanglef::with_tuple(Vector2::zero(), builder.enemy_dimensions);
        let mut game_object = GameObject::new(2, rectangle, builder.enemy_speed());
        game_object.set_move_direction(1, 1);
        Self {
            width: builder.enemy_dimensions.0,
            max_x: builder.dimensions.0,
            game_object,
        }
    }

    pub fn tick(&mut self) {
        self.game_object.tick();

        let pos = self.game_object.get_position();
        let max_x = (self.max_x - self.width) as f32;

        if pos.x < 0.0 || pos.x > max_x {
            self.game_object.mult_move_direction(-1, 1)
        }

        self.game_object.set_position(f32::max(0.0, f32::min(pos.x, max_x)), pos.y);
    }

    pub fn render<T>(&self, renderer: &mut T) where T: GameRenderer {
        self.game_object.render(renderer);
    }
}