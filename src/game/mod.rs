use std::cell::RefCell;
use std::rc::Rc;

use raylib::consts::*;

use builder::GameBuilder;
use enemy_ship::EnemyShip;
use input::InputRef;
use player_ship::PlayerShip;
use renderer::GameRenderer;

use crate::game::bullets_manager::BulletsManager;
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
            player_ship: Rc::new(RefCell::new(player_ship)),
            enemy_ship,
            enemies_enabled: builder.enemy_speed().x > 0.0,
            bullets_manager: BulletsManager::from_game_builder(builder)
        }
    }

    pub fn tick(&mut self) {
        if self.input.borrow().is_key_down(KEY_LEFT) {
            self.player_ship.borrow_mut().move_left();
        } else if self.input.borrow().is_key_down(KEY_RIGHT) {
            self.player_ship.borrow_mut().move_right();
        } else { self.player_ship.borrow_mut().stop() }

        if self.input.borrow().is_key_down(KEY_SPACE) {
            self.bullets_manager.spawn_bullet_at(self.player_ship.borrow().bullet_spawn_position());
        } else {
            self.bullets_manager.reset();
        }
        self.player_ship.borrow_mut().tick();
        self.enemy_ship.tick();
        self.bullets_manager.tick();
    }

    pub fn render<T>(&self, renderer: &mut T) where T: GameRenderer {
        renderer.clear();
        if self.enemies_enabled { self.enemy_ship.render(renderer) }
        self.player_ship.borrow().render(renderer);
        self.bullets_manager.render_bullets(renderer);
    }
}
