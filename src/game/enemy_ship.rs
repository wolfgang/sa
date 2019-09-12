use crate::game::builder::GameBuilder;
use crate::game::positioned::Positioned;
use crate::game::renderer::GameRenderer;

pub struct EnemyShip {
    speed: (f32, f32),
    x: i32,
    y: i32,
}

impl EnemyShip {
    pub fn from_game_builder(builder: &GameBuilder) -> Self {
        Self {
            speed: builder.enemy_speed(),
            x: 0,
            y: 0,
        }
    }

    pub fn tick(&mut self) {
        self.x += self.speed.0 as i32;
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