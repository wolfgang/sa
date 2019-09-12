use std::cmp::{max, min};

use crate::game::builder::GameBuilder;
use crate::game::positioned::Positioned;
use crate::game::renderer::GameRenderer;

pub struct EnemyShip {
    speed: (f32, f32),
    x: i32,
    y: i32,
    width: u32,
    max_x: u32,
    move_direction: i32,
}

impl EnemyShip {
    pub fn from_game_builder(builder: &GameBuilder) -> Self {
        Self {
            speed: builder.enemy_speed(),
            x: 0,
            y: 0,
            width: builder.enemy_dimensions.0,
            max_x: builder.dimensions.0,
            move_direction: 1,
        }
    }

    pub fn tick(&mut self) {
        let mut new_x = self.x + (self.move_direction * self.speed.0 as i32) as i32;
        let max_x = (self.max_x - self.width) as i32;
        if new_x >= max_x || new_x <= 0 {
            new_x = min(max_x, max(new_x, 0));
            self.move_direction *= -1;
        }
        self.x = new_x;
    }

    pub fn render<T>(&self, renderer: &mut T) where T: GameRenderer {
        renderer.draw_sprite_obj(2, self)
    }
}

impl Positioned for EnemyShip {
    fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }
}