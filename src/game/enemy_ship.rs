use crate::game::builder::GameBuilder;
use crate::game::game_object::GameObject;
use crate::game::renderer::GameRenderer;

pub struct EnemyShip {
    width: u32,
    max_x: u32,
    game_object: GameObject
}

impl EnemyShip {
    pub fn from_game_builder(builder: &GameBuilder) -> Self {
        let mut game_object = GameObject::new(2, 0, 0, builder.enemy_speed());
        game_object.set_move_direction(1, 1);
        Self {
            width: builder.enemy_dimensions.0,
            max_x: builder.dimensions.0,
            game_object
        }
    }

    pub fn tick(&mut self) {
        self.game_object.tick();

        let (x, y) = self.game_object.get_position();

        if x <= 0 {
            self.game_object.set_position(0, y);
            self.game_object.set_move_direction(1, 1);
        }

        let max_x = (self.max_x - self.width) as i32;
        if x >= max_x {
            self.game_object.set_position(max_x, y);
            self.game_object.set_move_direction(-1, 1);
        }
    }

    pub fn render<T>(&self, renderer: &mut T) where T: GameRenderer {
        self.game_object.render(renderer);
    }
}