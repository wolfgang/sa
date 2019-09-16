use crate::game::builder::GameBuilder;
use crate::game::geometry::{Rectanglef, Vector2};
use crate::game::moving_sprite::MovingSprite;
use crate::game::renderer::GameRenderer;

pub struct EnemyShip {
    screen_rect: Rectanglef,
    game_object: MovingSprite,
}

impl EnemyShip {
    pub fn from_game_builder(builder: &GameBuilder) -> Self {
        let go_rect = Rectanglef::with_tuple(Vector2::zero(), builder.enemy_dimensions);
        let mut game_object = MovingSprite::new(2, go_rect, builder.enemy_speed());
        game_object.set_move_direction(1, 1);
        Self {
            game_object,
            screen_rect: builder.screen_rect()
        }
    }

    pub fn tick(&mut self) {
        self.game_object.tick();

        if !self.game_object.is_inside_of(&self.screen_rect) {
            self.game_object.snap_to(&self.screen_rect);
            self.game_object.mult_move_direction(-1, 1);
        }

    }

    pub fn render<T>(&self, renderer: &mut T) where T: GameRenderer {
        renderer.draw_sprite_obj(&self.game_object)
    }
}