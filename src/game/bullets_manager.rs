use std::cell::RefCell;
use std::rc::Rc;

use crate::game::player_bullet::PlayerBullet;
use crate::game::player_ship::PlayerShip;
use crate::game::renderer::GameRenderer;

pub struct BulletsManager {
    player_ship: Rc<RefCell<PlayerShip>>,
    bullets: Vec<PlayerBullet>,
    bullet_dimensions: (u32, u32),
    last_bullet_tick: u32,
    current_tick: u32,
    fps: u32,
    bullet_speed: u32

}

impl BulletsManager {
    pub fn new(player_ship: Rc<RefCell<PlayerShip>>, fps: u32, bullet_dimensions: (u32, u32), bullet_speed: u32) -> Self {
        BulletsManager {
            player_ship: player_ship.clone(),
            bullets: Vec::with_capacity(10),
            bullet_dimensions,
            fps,
            last_bullet_tick: 0,
            current_tick: 1000,
            bullet_speed
        }
    }

    pub fn tick(&mut self) {
        for bullet in &mut self.bullets {
            bullet.tick();
        }
        self.current_tick += 1;
    }

    pub fn reset(&mut self) {
        self.last_bullet_tick = 0;
    }

    pub fn spawn_bullet(&mut self) {
        let seconds_since_last = (self.current_tick - self.last_bullet_tick) as f32 / self.fps as f32;

        if seconds_since_last >= 0.5 {
            self.last_bullet_tick = self.current_tick;
            let (bullet_width, bullet_height) = self.bullet_dimensions;
            let (x, y) = self.player_ship.borrow().bullet_spawn_position(bullet_width as i32, bullet_height as i32);
            self.bullets.push(PlayerBullet::new(x, y, self.fps, self.bullet_speed))
        }
    }

    pub fn render_bullets(&self, renderer: &mut dyn GameRenderer) {
        for bullet in &self.bullets {
            bullet.render(renderer);
        }
    }
}