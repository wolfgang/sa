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


pub struct GameBuilder {}

impl GameBuilder {
    pub fn with_dimensions(&mut self, width: u32, height: u32) -> &mut Self {
        self
    }

    pub fn with_ship_dimensions(&mut self, width: u32, height: u32) -> &mut Self {
        self
    }

    pub fn build(&self) -> Game {
        let mut world = World::new();
        world.register::<Geometry>();
        world.create_entity().with(Geometry { x: 3, y: 4, width: 3, height: 1 }).build();
        Game { world }
    }
}

pub struct Game { pub world: World }

impl Game {
    pub fn init() -> GameBuilder {
        GameBuilder {}
    }

    pub fn render<T>(&self, renderer: &mut T) where T: GameRenderer {
        let gos = self.world.read_storage::<Geometry>();

        for geometry in gos.join() {
            renderer.draw_sprite(0, geometry.x, geometry.y, geometry.width, geometry.height);
        }

    }
}