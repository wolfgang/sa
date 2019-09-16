use builder::GameBuilder;
use enemy_ship::EnemyShip;
use player_ship::PlayerShip;
use renderer::GameRenderer;

use crate::game::game_object::{GameObjectRef, NullGameObject};
use crate::game::game_objects_manager::GameObjectsManager;
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
pub mod game_objects_manager;

pub struct Game {
    player_controller: PlayerController,
    game_objects_manager: GameObjectsManager
}

impl Game {
    pub fn init() -> GameBuilder {
        GameBuilder::new()
    }

    pub fn from_builder(builder: &GameBuilder) -> Self {
        let player_ship = PlayerShip::from_game_builder(builder);
        let enemies_enabled = builder.enemy_speed().x > 0.0;
        let enemy_ship = if enemies_enabled { EnemyShip::from_game_builder(builder) } else { NullGameObject::new_rc() };

        let mut game_objects_manager = GameObjectsManager::new();
        game_objects_manager.add(player_ship.clone() as GameObjectRef);
        game_objects_manager.add(enemy_ship);

        Game {
            player_controller: PlayerController::new(builder, player_ship.clone()),
            game_objects_manager
        }
    }

    pub fn tick(&mut self) {
        self.player_controller.tick(&mut self.game_objects_manager);
        self.game_objects_manager.tick();

    }

    pub fn render<T>(&self, renderer: &mut T) where T: GameRenderer {
        renderer.clear();
        self.game_objects_manager.render(renderer);
    }
}
