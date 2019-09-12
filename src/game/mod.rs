use raylib::consts::*;

use builder::GameBuilder;
use enemy_ship::EnemyShip;
use input::InputRef;
use player_ship::PlayerShip;
use renderer::GameRenderer;

use crate::game::bullets_manager::BulletsManager;

pub mod builder;
pub mod input;
pub mod renderer;
pub mod positioned;
pub mod player_ship;
pub mod player_bullet;
pub mod bullets_manager;
pub mod enemy_ship;
pub mod game_object;

pub struct Game {
    input: InputRef,
    player_ship: PlayerShip,
    enemy_ship: EnemyShip,
    bullets_manager: BulletsManager,
    enemies_enabled: bool
}

impl Game {
    pub fn init() -> GameBuilder {
        GameBuilder::new()
    }

    pub fn from_builder(builder: &GameBuilder) -> Self {
        let player_ship = PlayerShip::from_game_builder(builder);
        let enemy_ship = EnemyShip::from_game_builder(builder);
        Game {
            input: builder.input.clone(),
            player_ship,
            enemy_ship,
            enemies_enabled: builder.enemy_speed().0 > 0,
            bullets_manager: BulletsManager::from_game_builder(builder)
        }
    }

    pub fn tick(&mut self) {
        if self.input.borrow().is_key_down(KEY_LEFT) {
            self.player_ship.move_left();
        } else if self.input.borrow().is_key_down(KEY_RIGHT) {
            self.player_ship.move_right();
        } else { self.player_ship.stop() }

        if self.input.borrow().is_key_down(KEY_SPACE) {
            self.bullets_manager.spawn_bullet_at(self.player_ship.bullet_spawn_position());
        } else {
            self.bullets_manager.reset();
        }
        self.player_ship.tick();
        self.enemy_ship.tick();
        self.bullets_manager.tick();
    }

    pub fn render<T>(&self, renderer: &mut T) where T: GameRenderer {
        renderer.clear();
        if self.enemies_enabled { self.enemy_ship.render(renderer) }
        self.player_ship.render(renderer);
        self.bullets_manager.render_bullets(renderer);
    }
}
