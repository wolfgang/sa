use crate::game::builder::GameBuilder;
use crate::game::game_object::GameObject;
use crate::game::geometry::{Rectanglef, Vector2};
use crate::game::renderer::GameRenderer;

pub struct EnemyShip {
    max_x: f32,
    game_object: GameObject,
}

impl EnemyShip {
    pub fn from_game_builder(builder: &GameBuilder) -> Self {
        let rectangle = Rectanglef::with_tuple(Vector2::zero(), builder.enemy_dimensions);
        let mut game_object = GameObject::new(2, rectangle, builder.enemy_speed());
        game_object.set_move_direction(1, 1);
        Self {
            max_x: builder.dimensions.0 as f32,
            game_object,
        }
    }

    pub fn tick(&mut self) {
        self.game_object.tick();

        if !self.game_object.is_inside_width(self.max_x) {
            self.game_object.mult_move_direction(-1, 1)
        }

        self.game_object.snap_to_width(self.max_x)
    }

    pub fn render<T>(&self, renderer: &mut T) where T: GameRenderer {
        self.game_object.render(renderer);
    }
}