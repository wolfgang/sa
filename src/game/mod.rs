use builder::GameBuilder;
use game_objects_manager::GameObjectsManager;
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
    game_objects_manager: GameObjectsManagerRef,
}

impl Game {
    pub fn init() -> GameBuilder {
        GameBuilder::new()
    }

    pub fn from_builder(builder: &GameBuilder) -> Self {
        let game_objects_manager = GameObjectsManager::new_rc(builder);
        let player_ship = game_objects_manager.borrow().player_ship();

        Game {
            player_controller: PlayerController::new(builder, player_ship, game_objects_manager.clone()),
            game_objects_manager: game_objects_manager.clone(),
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

