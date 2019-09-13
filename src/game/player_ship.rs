use crate::game::builder::GameBuilder;
use crate::game::game_object::GameObject;
use crate::game::geometry::{Rectanglef, Vector2};
use crate::game::renderer::GameRenderer;

pub struct PlayerShip {
    width: u32,
    screen_rect: Rectanglef,
    game_object: GameObject,
}

impl PlayerShip {
    pub fn from_game_builder(builder: &GameBuilder) -> PlayerShip {
        let (width, height) = builder.ship_dimensions;
        let (game_width, game_height) = builder.dimensions;
        let ship_x = game_width / 2 - width / 2;
        let ship_y = game_height - height;

        let go_rect = Rectanglef::with_tuple(Vector2::with(ship_x as f32, ship_y as f32), builder.ship_dimensions);
        Self {
            width,
            game_object: GameObject::new(0, go_rect, builder.ship_speed()),
            screen_rect: builder.screen_rect()
        }
    }

    pub fn render<T>(&self, renderer: &mut T) where T: GameRenderer {
        self.game_object.render(renderer)
    }

    pub fn move_left(&mut self) {
        self.game_object.set_move_direction(-1, 0);
    }

    pub fn move_right(&mut self) {
        self.game_object.set_move_direction(1, 0);
    }

    pub fn stop(&mut self) {
        self.game_object.set_move_direction(0, 0);
    }

    pub fn tick(&mut self) {
        self.game_object.tick();
        self.game_object.snap_to_width(self.screen_rect.width);
    }

    pub fn bullet_spawn_position(&self) -> (f32, f32) {
        let pos = self.game_object.get_position();
        (pos.x + self.width as f32 / 2.0, pos.y)
    }
}

