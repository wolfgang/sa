use raylib::consts::*;

use super::builder::GameBuilder;
use super::bullets_manager::BulletsManager;
use super::game_objects_manager::GameObjectsManagerRef;
use super::input::InputRef;
use super::objects::player_ship::PlayerShipRef;

pub struct PlayerController {
    input: InputRef,
    player_ship: PlayerShipRef,
    bullets_manager: BulletsManager,
    game_objects_manager: GameObjectsManagerRef,
    current_tick: u32,
}

impl PlayerController {
    pub fn new(builder: &GameBuilder, player_ship: PlayerShipRef, game_objects_manager: GameObjectsManagerRef) -> Self {
        Self {
            input: builder.input.clone(),
            bullets_manager: BulletsManager::from_game_builder(builder, player_ship.clone()),
            game_objects_manager,
            player_ship,
            current_tick: 1000,
        }
    }

    pub fn tick(&mut self) {
        self.current_tick += 1;
        if self.input.borrow().is_key_down(KEY_LEFT) {
            self.player_ship.borrow_mut().move_left();
        } else if self.input.borrow().is_key_down(KEY_RIGHT) {
            self.player_ship.borrow_mut().move_right();
        } else { self.player_ship.borrow_mut().stop() }

        if self.input.borrow().is_key_down(KEY_SPACE) {
            let bullet = self.bullets_manager.spawn_bullet(self.current_tick, self.game_objects_manager.clone());
            if bullet.is_some() { self.game_objects_manager.borrow_mut().add(bullet.unwrap()) };
        } else {
            self.bullets_manager.reset();
        }
    }
}

