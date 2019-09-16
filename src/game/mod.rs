use builder::GameBuilder;
use enemy_ship::EnemyShip;
use player_ship::PlayerShip;
use renderer::GameRenderer;

use crate::game::game_object::{GameObjectRef, NullGameObject};
use crate::game::player_controller::PlayerController;

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
pub mod player_controller;

pub struct Game {
    player_controller: PlayerController,
    game_objects: Vec<GameObjectRef>
}

impl Game {
    pub fn init() -> GameBuilder {
        GameBuilder::new()
    }

    pub fn from_builder(builder: &GameBuilder) -> Self {
        let player_ship = PlayerShip::from_game_builder(builder);
        let enemies_enabled = builder.enemy_speed().x > 0.0;
        let enemy_ship = if enemies_enabled { EnemyShip::from_game_builder(builder) } else { NullGameObject::new_rc() };
        let mut game_objects = Vec::with_capacity(10);

        game_objects.push(player_ship.clone() as GameObjectRef);
        game_objects.push(enemy_ship);

        Game {
            player_controller: PlayerController::new(builder, player_ship.clone()),
            game_objects
        }
    }

    pub fn tick(&mut self) {
        let bullet = self.player_controller.tick();
        if bullet.is_some() { self.game_objects.push(bullet.unwrap()) }

        for go in self.game_objects.iter() {
            go.borrow_mut().tick();
        }

        self.game_objects.retain(|go| { go.borrow().is_alive() });
    }

    pub fn render<T>(&self, renderer: &mut T) where T: GameRenderer {
        renderer.clear();
        for go in self.game_objects.iter() {
            go.borrow().render(renderer);
        }
    }
}
