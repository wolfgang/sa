use raylib::consts::*;

use crate::game::input::InputRef;
use crate::game::player_ship::PlayerShipRef;

pub struct PlayerController {
    input: InputRef,
    player_ship: PlayerShipRef,
}

impl PlayerController {
    pub fn new(input: InputRef, player_ship: PlayerShipRef) -> Self {
        Self {
            input,
            player_ship,
        }
    }

    pub fn tick(&mut self) {
        if self.input.borrow().is_key_down(KEY_LEFT) {
            self.player_ship.borrow_mut().move_left();
        } else if self.input.borrow().is_key_down(KEY_RIGHT) {
            self.player_ship.borrow_mut().move_right();
        } else { self.player_ship.borrow_mut().stop() }
    }
}

