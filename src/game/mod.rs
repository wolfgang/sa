use raylib::consts::*;

use builder::GameBuilder;
use input::InputRef;
use renderer::GameRenderer;

pub mod builder;
pub mod input;
pub mod renderer;

pub struct Game {
    dimensions: (u32, u32),
    ship_dimensions: (u32, u32),
    ship_position: (i32, i32),
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
            ship_position: (ship_x as i32, ship_y as i32),
            ship_speed: builder.ship_speed,
        }
    }

    pub fn tick(&mut self) {
        if self.input.borrow().is_key_down(KEY_LEFT) {
            self.ship_position.0 -= (self.ship_speed / self.fps) as i32;
            if self.ship_position.0 < 0 { self.ship_position.0 = 0 }
        }

        if self.input.borrow().is_key_down(KEY_RIGHT) {
            self.ship_position.0 += (self.ship_speed / self.fps) as i32;
            if self.ship_position.0 >= (self.dimensions.0 - self.ship_dimensions.0) as i32 {
                self.ship_position.0 = (self.dimensions.0 - self.ship_dimensions.0) as i32
            }
        }
    }

    pub fn render(&self, renderer: &mut dyn GameRenderer) {
        renderer.clear();
        renderer.draw_sprite(0, self.ship_position.0, self.ship_position.1);
    }
}