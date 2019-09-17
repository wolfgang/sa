use std::cell::RefCell;
use std::rc::Rc;

use crate::core::geometry::{Rectanglef, Vector2};
use crate::game::builder::GameBuilder;

use super::game_object::{GameObject, GameObjectRef};
use super::moving_sprite::MovingSprite;

trait Behavior {
    fn tick(&self, go: &mut EnemyShip);
}

struct EnemyShipBehavior {}

impl Behavior for EnemyShipBehavior {
    fn tick(&self, go: &mut EnemyShip) {
        if !go.moving_sprite.is_inside_of(&go.screen_rect) {
            go.moving_sprite.snap_to(&go.screen_rect);
            go.moving_sprite.mult_move_direction(-1, 1);
        }
    }
}


pub struct EnemyShip {
    screen_rect: Rectanglef,
    moving_sprite: MovingSprite,
    is_alive: bool,
    behavior: Rc<RefCell<dyn Behavior>>
}

impl EnemyShip {
    pub fn from_game_builder(builder: &GameBuilder) -> GameObjectRef {
        let go_rect = Rectanglef::with_tuple(Vector2::zero(), builder.enemy_dimensions);
        let mut moving_sprite = MovingSprite::new(2, go_rect, builder.enemy_speed());
        moving_sprite.set_move_direction(1, 1);
        Rc::new(RefCell::new(Self {
            moving_sprite,
            screen_rect: builder.screen_rect(),
            is_alive: true,
            behavior: Rc::new(RefCell::new(EnemyShipBehavior {}))
        }))
    }
}

impl GameObject for EnemyShip {
    fn tick(&mut self) {
        self.moving_sprite.tick();
        self.behavior.clone().borrow().tick(self);

    }

    fn collider(&self) -> &MovingSprite {
        &self.moving_sprite
    }

    fn is_alive(&self) -> bool {
        self.is_alive
    }


    fn on_collision(&mut self) {
        self.is_alive = false;
    }
}