use raylib::consts::*;

use builder::GameBuilder;
use input::InputRef;
use player_bullet::PlayerBullet;
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

pub struct Game {
    input: InputRef,
    fps: u32,
    ship_speed: u32,
    player_ship: PlayerShip,
    player_bullets: Vec<PlayerBullet>,
    bullet_dimensions: (u32, u32),
    last_bullet_tick: u32,
    current_tick: u32
}

impl Game {
    pub fn init() -> GameBuilder {
        GameBuilder::new()
    }

    pub fn from_builder(builder: &GameBuilder) -> Self {
        Game {
            input: builder.input.clone(),
            fps: builder.fps,
            player_ship: PlayerShip::new(builder.ship_dimensions, builder.dimensions),
            ship_speed: builder.ship_speed,
            player_bullets: Vec::with_capacity(10),
            bullet_dimensions: builder.bullet_dimensions,
            last_bullet_tick: 0,
            current_tick: 1000,
        }
    }

    pub fn tick(&mut self) {
        let offset = (self.ship_speed / self.fps) as i32;
        if self.input.borrow().is_key_down(KEY_LEFT) {
            self.player_ship.move_horizontally(-1 * offset);
        }

        if self.input.borrow().is_key_down(KEY_RIGHT) {
            self.player_ship.move_horizontally(offset);
        }

        if self.input.borrow().is_key_down(KEY_SPACE) {
            let seconds_since_last = (self.current_tick - self.last_bullet_tick) as f32 / self.fps as f32;

            if seconds_since_last >= 0.5 {
                self.last_bullet_tick = self.current_tick;
                let (bullet_width, bullet_height) = self.bullet_dimensions;
                let (x, y) = self.player_ship.bullet_spawn_position(bullet_width as i32, bullet_height as i32);
                self.player_bullets.push(PlayerBullet::new(x, y))
            }
        }

        self.current_tick += 1;
    }

    pub fn render(&self, renderer: &mut dyn GameRenderer) {
        renderer.clear();
        self.player_ship.render(renderer);

        for bullet in &self.player_bullets {
            bullet.render(renderer);
        }
    }
}