use crate::game::builder::GameBuilder;
use crate::game::geometry::{Rectanglef, Vector2, Vector2f};
use crate::game::player_bullet::PlayerBullet;
use crate::game::renderer::GameRenderer;

pub struct BulletsManager {
    bullets: Vec<PlayerBullet>,
    last_bullet_tick: u32,
    current_tick: u32,
    autofire_ticks: u32,
    bullet_speed: Vector2f,
    bullet_dimensions: (u32, u32)
}

impl BulletsManager {
    pub fn from_game_builder(builder: &GameBuilder) -> Self {
        BulletsManager {
            bullet_speed: builder.bullet_speed(),
            autofire_ticks: builder.autofire_ticks(),
            bullets: Vec::with_capacity(10),
            last_bullet_tick: 0,
            current_tick: 1000,
            bullet_dimensions: builder.bullet_dimensions
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

    pub fn spawn_bullet_at(&mut self, position: (f32, f32)) {

        let ticks_since_last = self.current_tick - self.last_bullet_tick;

        if ticks_since_last >= self.autofire_ticks {
            self.last_bullet_tick = self.current_tick;
            let rect = Rectanglef::with_tuple(Vector2::from(position), self.bullet_dimensions);
            self.bullets.push(PlayerBullet::new(rect, self.bullet_speed))
        }
    }

    pub fn render_bullets<T>(&self, renderer: &mut T) where T: GameRenderer {
        for bullet in &self.bullets {
            bullet.render(renderer);
        }
    }
}
