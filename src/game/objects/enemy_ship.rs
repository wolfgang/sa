use std::cell::RefCell;
use std::rc::Rc;

use crate::core::geometry::{Rectanglef, Vector2};
use crate::game::builder::GameBuilder;
use crate::gfx::renderer::GameRenderer;

use super::game_object::{GameObject, GameObjectRef};
use super::moving_sprite::MovingSprite;

pub struct EnemyShip {
    screen_rect: Rectanglef,
    moving_sprite: MovingSprite,
    is_alive: bool
}

impl EnemyShip {
    pub fn from_game_builder(builder: &GameBuilder) -> GameObjectRef {
        let go_rect = Rectanglef::with_tuple(Vector2::zero(), builder.enemy_dimensions);
        let mut moving_sprite = MovingSprite::new(2, go_rect, builder.enemy_speed());
        moving_sprite.set_move_direction(1, 1);
        Rc::new(RefCell::new(Self {
            moving_sprite,
            screen_rect: builder.screen_rect(),
            is_alive: true
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

    fn is_alive(&self) -> bool {
        self.is_alive
    }

    fn is_target(&self) -> bool {
        true
    }

    fn process_collision(&mut self, other: &MovingSprite) -> bool {
        if other.intersects_with(&self.moving_sprite) {
            self.is_alive = false;
            return true;
        }
        false
    }

    fn on_collision(&mut self) {
        self.is_alive = false;
    }


    fn collider(&self) -> &MovingSprite {
        &self.moving_sprite
    }
}