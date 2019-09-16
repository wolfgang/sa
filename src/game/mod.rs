use raylib::consts::*;

use builder::GameBuilder;
use enemy_ship::EnemyShip;
use input::InputRef;
use player_ship::PlayerShip;
use renderer::GameRenderer;

use crate::game::bullets_manager::BulletsManager;
use crate::game::game_object::{GameObjectRef, NulLGameObject};
use crate::game::player_ship::PlayerShipRef;

pub mod geometry;
pub mod builder;
pub mod input;
pub mod renderer;
pub mod player_ship;
pub mod player_bullet;
pub mod bullets_manager;
pub mod enemy_ship;
pub mod moving_sprite;
pub mod sprite;
pub mod game_object;

pub struct Game {
    input: InputRef,
    player_ship: PlayerShipRef,
    bullets_manager: BulletsManager,
    game_objects: Vec<GameObjectRef>,
}

impl Game {
    pub fn init() -> GameBuilder {
        GameBuilder::new()
    }

    pub fn from_builder(builder: &GameBuilder) -> Self {
        let player_ship = PlayerShip::from_game_builder(builder);
        let enemies_enabled = builder.enemy_speed().x > 0.0;
        let enemy_ship = if enemies_enabled { EnemyShip::from_game_builder(builder) } else { NulLGameObject::new_rc() };
        let mut game_objects = Vec::with_capacity(10);

        game_objects.push(player_ship.clone() as GameObjectRef);
        game_objects.push(enemy_ship);

        Game {
            input: builder.input.clone(),
            player_ship: player_ship.clone(),
            bullets_manager: BulletsManager::from_game_builder(builder, player_ship.clone()),
            game_objects,
        }
    }


    pub fn tick(&mut self) {
        if self.input.borrow().is_key_down(KEY_LEFT) {
            self.player_ship.borrow_mut().move_left();
        } else if self.input.borrow().is_key_down(KEY_RIGHT) {
            self.player_ship.borrow_mut().move_right();
        } else { self.player_ship.borrow_mut().stop() }

        if self.input.borrow().is_key_down(KEY_SPACE) {
            let bullet = self.bullets_manager.spawn_bullet();
            if bullet.is_some() { self.game_objects.push(bullet.unwrap()) }
        } else {
            self.bullets_manager.reset();
        }

        for go in self.game_objects.iter() {
            go.borrow_mut().tick();
        }

        self.game_objects.retain(|go| { go.borrow_mut().is_alive() });
        self.bullets_manager.tick();
    }

    pub fn render<T>(&self, renderer: &mut T) where T: GameRenderer {
        renderer.clear();
        for go in self.game_objects.iter() {
            go.borrow().render(renderer);
        }

    }
}
