use raylib::consts::{KEY_LEFT, KEY_RIGHT};
use specs::prelude::*;
use specs_derive::Component;

use crate::game::input::{InputNull, InputRef};
use crate::gfx::game_renderer::GameRenderer;

#[derive(Component)]
struct Geometry {
    x: i32,
    y: i32,
    width: u32,
    height: u32,
}

#[derive(Component)]
struct Mover {
    speed: u32,
    current_speed: i32,
}


pub struct GameBuilder {
    dimensions: (u32, u32),
    ship_dimensions: (u32, u32),
    ship_speed: u32,
    input: InputRef,
}

impl GameBuilder {
    pub fn new() -> Self {
        Self {
            dimensions: (0, 0),
            ship_dimensions: (0, 0),
            ship_speed: 0,
            input: InputNull::new_rc(),
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

    pub fn with_ship_speed(&mut self, speed: u32) -> &mut Self {
        self.ship_speed = speed;
        self
    }

    pub fn with_input(&mut self, input: InputRef) -> &mut Self {
        self.input = input;
        self
    }

    pub fn build(&self) -> Game {
        let mut world = World::new();
        world.register::<Geometry>();
        world.register::<Mover>();

        let (ship_width, ship_height) = self.ship_dimensions;
        let (width, height) = self.dimensions;
        let x = width / 2 - ship_width / 2;
        let y = height - ship_height;
        world.create_entity()
            .with(Geometry { x: x as i32, y: y as i32, width: ship_width, height: ship_height })
            .with(Mover { speed: self.ship_speed, current_speed: 0 })
            .build();
        Game { world, input: self.input.clone() }
    }
}

pub struct Game {
    world: World,
    input: InputRef,
}

impl Game {
    pub fn init() -> GameBuilder {
        GameBuilder::new()
    }

    pub fn tick(&mut self) {
        let mut gos = self.world.write_storage::<Geometry>();
        let mut movers = self.world.write_storage::<Mover>();

        for mover in (&mut movers).join() {
            if self.input.borrow().is_key_down(KEY_LEFT) {
                mover.current_speed = mover.speed as i32 * -1;
            } else if self.input.borrow().is_key_down(KEY_RIGHT) {
                mover.current_speed = mover.speed as i32;
            } else {
                mover.current_speed = 0;
            }
        }

        for (geometry, mover) in (&mut gos, &mut movers).join() {
            geometry.x += mover.current_speed;
        }
    }

    pub fn render<T>(&self, renderer: &mut T) where T: GameRenderer {
        renderer.clear();
        let gos = self.world.read_storage::<Geometry>();
        for geometry in (&gos).join() {
            renderer.draw_sprite(0, geometry.x, geometry.y, geometry.width, geometry.height);
        }
    }
}