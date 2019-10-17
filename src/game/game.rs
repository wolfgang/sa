use specs::prelude::*;
use specs_derive::Component;

use crate::gfx::game_renderer::GameRenderer;

#[derive(Component)]
struct Geometry {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}


#[derive(Default)]
pub struct GameBuilder {
    dimensions: (u32, u32),
    ship_dimensions: (u32, u32),
}

impl GameBuilder {
    pub fn with_dimensions(&mut self, width: u32, height: u32) -> &mut Self {
        self.dimensions = (width, height);
        self
    }

    pub fn with_ship_dimensions(&mut self, width: u32, height: u32) -> &mut Self {
        self.ship_dimensions = (width, height);
        self
    }

    pub fn build(&self) -> Game {
        let mut world = World::new();
        world.register::<Geometry>();

        let (ship_width, ship_height) = self.ship_dimensions;
        let (width, height) = self.dimensions;
        let x = width / 2 - ship_width / 2;
        let y = height - ship_height;
        world.create_entity().with(Geometry { x, y, width: ship_width, height: ship_height }).build();
        Game { world }
    }
}

pub struct Game { pub world: World }

impl Game {
    pub fn init() -> GameBuilder {
        GameBuilder::default()
    }

    pub fn render<T>(&self, renderer: &mut T) where T: GameRenderer {
        let gos = self.world.read_storage::<Geometry>();

        for geometry in gos.join() {
            renderer.draw_sprite(0, geometry.x, geometry.y, geometry.width, geometry.height);
        }
    }
}