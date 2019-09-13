use crate::game::Game;
use crate::game::input::{InputNull, InputRef};

#[derive(Clone)]
pub struct GameBuilder {
    pub dimensions: (u32, u32),
    pub ship_dimensions: (u32, u32),
    pub bullet_dimensions: (u32, u32),
    pub enemy_dimensions: (u32, u32),
    pub input: InputRef,
    fps: u32,
    ship_speed: u32,
    bullet_speed: u32,
    enemy_speed: (u32, u32),
}

impl GameBuilder {
    pub fn new() -> Self {
        GameBuilder {
            input: InputNull::new_rc(),
            dimensions: (0, 0),
            ship_dimensions: (0, 0),
            bullet_dimensions: (0, 0),
            fps: 30,
            ship_speed: 0,
            bullet_speed: 0,
            enemy_speed: (0, 0),
            enemy_dimensions: (0, 0),
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

    pub fn with_bullet_dimensions(&mut self, width: u32, height: u32) -> &mut Self {
        self.bullet_dimensions = (width, height);
        self
    }

    pub fn with_enemy_dimensions(&mut self, width: u32, height: u32) -> &mut Self {
        self.enemy_dimensions = (width, height);
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

    pub fn with_bullet_speed(&mut self, ship_speed: u32) -> &mut Self {
        self.bullet_speed = ship_speed;
        self
    }

    pub fn with_enemy_speed(&mut self, dx: u32, dy: u32) -> &mut Self {
        self.enemy_speed = (dx, dy);
        self
    }

    pub fn ship_speed(&self) -> u32 {
        self.ship_speed / self.fps
    }

    pub fn bullet_speed(&self) -> u32 {
        self.bullet_speed / self.fps
    }

    pub fn enemy_speed(&self) -> (f32, f32) {
        (self.enemy_speed.0 as f32 / self.fps as f32, self.enemy_speed.1 as f32 / self.fps as f32)
    }

    pub fn autofire_ticks(&self) -> u32 {
        (0.5 * (self.fps as f32)) as u32
    }

    pub fn build(&self) -> Game {
        Game::from_builder(self)
    }
}
