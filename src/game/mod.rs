use raylib::consts::*;

use builder::GameBuilder;
use input::InputRef;
use renderer::GameRenderer;

use crate::game::player_ship::PlayerShip;

pub mod builder;
pub mod input;
pub mod renderer;
pub mod player_ship;

pub struct Game {
    input: InputRef,
    fps: u32,
    ship_speed: u32,
    player_ship: PlayerShip
}

impl Game {
    pub fn init() -> GameBuilder {
        GameBuilder::new()
    }

    pub fn from_builder(builder: &GameBuilder) -> Self {
        Game {
            input: builder.input.clone(),
            fps: builder.fps,
            player_ship: PlayerShip::new(builder.ship_dimensions, builder.dimensions),
            ship_speed: builder.ship_speed,
        }
    }

    pub fn tick(&mut self) {
        let offset = (self.ship_speed / self.fps) as i32;
        if self.input.borrow().is_key_down(KEY_LEFT) {
            self.player_ship.move_horizontally(-1 * offset);
        }

        if self.input.borrow().is_key_down(KEY_RIGHT) {
            self.player_ship.move_horizontally(offset);
        }
    }

    pub fn render(&self, renderer: &mut dyn GameRenderer) {
        renderer.clear();
        self.player_ship.render(renderer);
    }
}