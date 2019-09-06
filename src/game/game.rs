use raylib::consts::{KEY_LEFT, KEY_RIGHT};
use raylib::Vector2;

use crate::game::input::{InputNull, InputRef};
use crate::gfx::game_renderer::GameRenderer;

#[derive(Clone)]
pub struct GameBuilder {
    pub dimensions: (u32, u32),
    pub ship_dimensions: (u32, u32),
    pub input: InputRef,
    pub fps: u32,
    pub ship_speed: u32
}

impl GameBuilder {
    pub fn new() -> Self {
        GameBuilder {
            input: InputNull::new_rc(),
            dimensions: (0, 0),
            ship_dimensions: (0, 0),
            fps: 30,
            ship_speed: 1,
        }
    }

    pub fn with_dimensions(&mut self, width: u32, height: u32) -> &mut Self {
        self.dimensions = (width, height);
        self
    }

    pub fn with_ship_dimensions(&mut self, width: u32, height: u32) -> &mut Self {
        self.ship_dimensions = (width, height);
        self
    }

    pub fn with_input(&mut self, input: InputRef) -> &mut Self {
        self.input = input;
        self
    }

    pub fn with_fps(&mut self, fps: u32) -> &mut Self {
        self.fps = fps;
        self
    }

    pub fn with_ship_speed(&mut self, ship_speed: u32) -> &mut Self {
        self.ship_speed = ship_speed;
        self
    }

    pub fn build(&self) -> Game {
        let ship_x = self.dimensions.0 / 2 - self.ship_dimensions.0 / 2;
        let ship_y = self.dimensions.1 - self.ship_dimensions.1;
        Game {
            input: self.input.clone(),
            fps: self.fps,
            dimensions: self.dimensions,
            ship_dimensions: self.ship_dimensions,
            ship_position: Vector2 { x: ship_x as f32, y: ship_y as f32 },
            ship_speed: self.ship_speed
        }
    }
}

pub struct Game {
    pub dimensions: (u32, u32),
    pub ship_dimensions: (u32, u32),
    pub ship_position: Vector2,
    pub input: InputRef,
    pub fps: u32,
    pub ship_speed: u32
}

impl Game {
    pub fn init() -> GameBuilder {
        GameBuilder::new()
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
        renderer.draw_sprite(0, self.ship_position.x as u32, self.ship_position.y as u32);
    }
}