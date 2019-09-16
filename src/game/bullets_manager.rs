use std::cell::RefCell;
use std::rc::Rc;

use crate::game::builder::GameBuilder;
use crate::game::game_object::GameObjectRef;
use crate::game::geometry::{Rectanglef, Vector2, Vector2f};
use crate::game::player_bullet::PlayerBullet;
use crate::game::player_ship::PlayerShipRef;

pub struct BulletsManager {
    last_bullet_tick: u32,
    current_tick: u32,
    player_ship: PlayerShipRef,
    autofire_ticks: u32,
    bullet_speed: Vector2f,
    bullet_dimensions: (u32, u32)
}

impl BulletsManager {
    pub fn from_game_builder(builder: &GameBuilder, player_ship: PlayerShipRef) -> Self {
        BulletsManager {
            bullet_speed: builder.bullet_speed(),
            autofire_ticks: builder.autofire_ticks(),
            player_ship,
            last_bullet_tick: 0,
            current_tick: 1000,
            bullet_dimensions: builder.bullet_dimensions
        }
    }

    pub fn tick(&mut self) {
        self.current_tick += 1;
    }

    pub fn reset(&mut self) {
        self.last_bullet_tick = 0;
    }

    pub fn spawn_bullet(&mut self) -> Option<GameObjectRef> {
        let position = self.player_ship.borrow().bullet_spawn_position();
        let ticks_since_last = self.current_tick - self.last_bullet_tick;
        if ticks_since_last >= self.autofire_ticks {
            self.last_bullet_tick = self.current_tick;

            let (bullet_width, bullet_height) = self.bullet_dimensions;
            let adjusted_pos = Vector2::with(position.x - bullet_width as f32 / 2.0, position.y - bullet_height as f32);

            let rect = Rectanglef::with_tuple(adjusted_pos, self.bullet_dimensions);
            return Some(Rc::new(RefCell::new(PlayerBullet::new(rect, self.bullet_speed))));
        }
        None
    }
}
