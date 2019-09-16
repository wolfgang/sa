use raylib::consts::*;

use crate::game::builder::GameBuilder;
use crate::game::bullets_manager::BulletsManager;
use crate::game::game_object::GameObjectRef;
use crate::game::input::InputRef;
use crate::game::player_ship::PlayerShipRef;

pub struct PlayerController {
    input: InputRef,
    player_ship: PlayerShipRef,
    bullets_manager: BulletsManager,
    current_tick: u32,
}

impl PlayerController {
    pub fn new(builder: &GameBuilder, player_ship: PlayerShipRef) -> Self {
        Self {
            input: builder.input.clone(),
            bullets_manager: BulletsManager::from_game_builder(builder, player_ship.clone()),
            player_ship,
            current_tick: 1000,
        }
    }

    pub fn tick(&mut self, game_objects: &mut Vec<GameObjectRef>) {
        self.current_tick += 1;
        if self.input.borrow().is_key_down(KEY_LEFT) {
            self.player_ship.borrow_mut().move_left();
        } else if self.input.borrow().is_key_down(KEY_RIGHT) {
            self.player_ship.borrow_mut().move_right();
        } else { self.player_ship.borrow_mut().stop() }

        if self.input.borrow().is_key_down(KEY_SPACE) {
            let bullet = self.bullets_manager.spawn_bullet(self.current_tick);
            if bullet.is_some() { game_objects.push(bullet.unwrap()) };
        } else {
            self.bullets_manager.reset();
        }
    }
}

