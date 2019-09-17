use builder::GameBuilder;
use game_objects_manager::GameObjectsManager;
use objects::enemy_ship::EnemyShip;
use objects::game_object::{GameObjectRef, NullGameObject};
use objects::player_ship::PlayerShip;
use player_controller::PlayerController;

use crate::game::game_objects_manager::GameObjectsManagerRef;
use crate::gfx::renderer::GameRenderer;

pub mod builder;
pub mod input;
pub mod bullets_manager;
pub mod player_controller;
pub mod game_objects_manager;
pub mod objects;

pub struct Game {
    player_controller: PlayerController,
    game_objects_manager: GameObjectsManagerRef
}

impl Game {
    pub fn init() -> GameBuilder {
        GameBuilder::new()
    }

    pub fn from_builder(builder: &GameBuilder) -> Self {
        let player_ship = PlayerShip::from_game_builder(builder);
        let enemies_enabled = builder.enemy_speed().x > 0.0;
        let enemy_ship = if enemies_enabled { EnemyShip::from_game_builder(builder) } else { NullGameObject::new_rc() };

        let game_objects_manager = GameObjectsManager::new_rc();
        game_objects_manager.borrow_mut().add(player_ship.clone() as GameObjectRef);
        game_objects_manager.borrow_mut().add(enemy_ship);

        Game {
            player_controller: PlayerController::new(builder, player_ship.clone(), game_objects_manager.clone()),
            game_objects_manager: game_objects_manager.clone()
        }
    }

    pub fn tick(&mut self) {
        self.player_controller.tick();
        self.game_objects_manager.borrow_mut().tick();
    }

    pub fn render<T>(&self, renderer: &mut T) where T: GameRenderer {
        renderer.clear();
        self.game_objects_manager.borrow().render(renderer);
    }
}
