use crate::game::game::Game;
use crate::game::input::{InputNull, InputRef};

#[derive(Clone)]
pub struct GameBuilder {
    pub dimensions: (u32, u32),
    pub ship_dimensions: (u32, u32),
    pub input: InputRef,
    pub fps: u32,
    pub ship_speed: u32,
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
        Game::from_builder(self)
    }
}
