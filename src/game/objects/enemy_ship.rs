use std::cell::RefCell;
use std::rc::Rc;

use crate::game::builder::GameBuilder;
use crate::game::geometry::{Rectanglef, Vector2};
use crate::game::renderer::GameRenderer;

use super::game_object::{GameObject, GameObjectRef};
use super::moving_sprite::MovingSprite;

pub struct EnemyShip {
    screen_rect: Rectanglef,
    moving_sprite: MovingSprite,
}

impl EnemyShip {
    pub fn from_game_builder(builder: &GameBuilder) -> GameObjectRef {
        let go_rect = Rectanglef::with_tuple(Vector2::zero(), builder.enemy_dimensions);
        let mut moving_sprite = MovingSprite::new(2, go_rect, builder.enemy_speed());
        moving_sprite.set_move_direction(1, 1);
        Rc::new(RefCell::new(Self {
            moving_sprite,
            screen_rect: builder.screen_rect()
        }))
    }
}

impl GameObject for EnemyShip {
    fn tick(&mut self) {
        self.moving_sprite.tick();

        if !self.moving_sprite.is_inside_of(&self.screen_rect) {
            self.moving_sprite.snap_to(&self.screen_rect);
            self.moving_sprite.mult_move_direction(-1, 1);
        }
    }

    fn render(&self, renderer: &mut dyn GameRenderer) {
        self.moving_sprite.render(renderer);
    }
}