use std::cell::RefCell;
use std::rc::Rc;

use crate::core::geometry::{Rectanglef, Vector2, Vector2f};
use crate::game::game_objects_manager::GameObjectsManagerRef;

use super::builder::GameBuilder;
use super::objects::game_object::GameObjectRef;
use super::objects::player_bullet::PlayerBullet;
use super::objects::player_ship::PlayerShipRef;

pub struct BulletsManager {
    last_bullet_tick: u32,
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
            bullet_dimensions: builder.bullet_dimensions
        }
    }

    pub fn reset(&mut self) {
        self.last_bullet_tick = 0;
    }

    pub fn spawn_bullet(&mut self, current_tick: u32, game_objects_manager: GameObjectsManagerRef) -> Option<GameObjectRef> {
        let ticks_since_last = current_tick - self.last_bullet_tick;
        if ticks_since_last >= self.autofire_ticks {
            self.last_bullet_tick = current_tick;
            let position = self.player_ship.borrow().bullet_spawn_position();
            let (bullet_width, bullet_height) = self.bullet_dimensions;
            let adjusted_pos = Vector2::with(position.x - bullet_width as f32 / 2.0, position.y - bullet_height as f32);

            let rect = Rectanglef::with_tuple(adjusted_pos, self.bullet_dimensions);
            return Some(Rc::new(RefCell::new(PlayerBullet::new(rect, self.bullet_speed, game_objects_manager))));
        }
        None
    }
}
