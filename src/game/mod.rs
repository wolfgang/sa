use raylib::consts::*;
use raylib::Vector2;

use builder::GameBuilder;
use input::InputRef;
use renderer::GameRenderer;

pub mod builder;
pub mod input;
pub mod renderer;

pub struct Game {
    dimensions: (u32, u32),
    ship_dimensions: (u32, u32),
    ship_position: Vector2,
    input: InputRef,
    fps: u32,
    ship_speed: u32,
}

impl Game {
    pub fn init() -> GameBuilder {
        GameBuilder::new()
    }

    pub fn from_builder(builder: &GameBuilder) -> Self {
        let ship_x = builder.dimensions.0 / 2 - builder.ship_dimensions.0 / 2;
        let ship_y = builder.dimensions.1 - builder.ship_dimensions.1;
        Game {
            input: builder.input.clone(),
            fps: builder.fps,
            dimensions: builder.dimensions,
            ship_dimensions: builder.ship_dimensions,
            ship_position: Vector2 { x: ship_x as f32, y: ship_y as f32 },
            ship_speed: builder.ship_speed,
        }
    }

    pub fn tick(&mut self) {
        if self.input.borrow().is_key_down(KEY_LEFT) {
            self.ship_position.x -= self.ship_speed as f32 / self.fps as f32;
            if self.ship_position.x < 0.0 { self.ship_position.x = 0.0 }
        }

        if self.input.borrow().is_key_down(KEY_RIGHT) {
            self.ship_position.x += self.ship_speed as f32 / self.fps as f32;
            if self.ship_position.x >= (self.dimensions.0 - self.ship_dimensions.0) as f32 {
                self.ship_position.x = (self.dimensions.0 - self.ship_dimensions.0) as f32
            }
        }
    }

    pub fn render(&self, renderer: &mut dyn GameRenderer) {
        renderer.clear();
        renderer.draw_sprite(0, self.ship_position.x as i32, self.ship_position.y as i32);
    }
}