use std::cell::RefCell;
use std::rc::Rc;

use crate::game::builder::GameBuilder;
use crate::game::player_bullet::PlayerBullet;
use crate::game::player_ship::{PlayerShip, PlayerShipRef};
use crate::game::renderer::GameRenderer;

pub struct BulletsManager {
    player_ship: Rc<RefCell<PlayerShip>>,
    bullets: Vec<PlayerBullet>,
    bullet_dimensions: (u32, u32),
    last_bullet_tick: u32,
    current_tick: u32,
    autofire_ticks: u32,
    bullet_speed: u32,
}

impl BulletsManager {
    pub fn from_game_builder(builder: &GameBuilder, player_ship: PlayerShipRef) -> Self {
        BulletsManager {
            bullet_speed: builder.bullet_speed(),
            autofire_ticks: builder.autofire_ticks(),
            bullet_dimensions: builder.bullet_dimensions,
            player_ship: player_ship.clone(),
            bullets: Vec::with_capacity(10),
            last_bullet_tick: 0,
            current_tick: 1000,
        }
    }

    pub fn tick(&mut self) {
        for bullet in &mut self.bullets { bullet.tick() }
        self.bullets.retain(|bullet| { bullet.is_alive() });
        self.current_tick += 1;
    }

    pub fn reset(&mut self) {
        self.last_bullet_tick = 0;
    }

    pub fn spawn_bullet(&mut self) {
        let ticks_since_last = self.current_tick - self.last_bullet_tick;

        if ticks_since_last >= self.autofire_ticks {
            self.last_bullet_tick = self.current_tick;
            let (bullet_width, bullet_height) = self.bullet_dimensions;
            let (x, y) = self.player_ship.borrow().bullet_spawn_position(bullet_width as i32, bullet_height as i32);
            self.bullets.push(PlayerBullet::new(x, y, self.bullet_speed))
        }
    }

    pub fn render_bullets(&self, renderer: &mut dyn GameRenderer) {
        for bullet in &self.bullets {
            bullet.render(renderer);
        }
    }
}