use std::cmp::{max, min};

use raylib::consts::{KEY_LEFT, KEY_RIGHT, KEY_SPACE};
use specs::prelude::*;
use specs_derive::Component;

use crate::game::input::{InputNull, InputRef};
use crate::gfx::game_renderer::GameRenderer;

pub mod input;

#[derive(Component)]
struct Geometry {
    x: i32,
    y: i32,
    width: u32,
    height: u32,
}

#[derive(Component)]
struct Velocity(i32, i32);

#[derive(Component)]
struct Sprite {
    id: u8
}

#[derive(Component, Default)]
#[storage(NullStorage)]
struct IsPlayer;

#[derive(Clone, Default)]
pub struct GameConfig {
    pub dimensions: (u32, u32),
    pub ship_dimensions: (u32, u32),
    pub ship_speed: u32,
    pub bullet_dimensions: (u32, u32),
    pub bullet_speed: u32,
}


#[derive(Clone)]
pub struct GameBuilder {
    input: InputRef,
    pub(crate) config: GameConfig,
}

impl GameBuilder {
    pub fn new() -> Self {
        Self {
            input: InputNull::new_rc(),
            config: GameConfig::default(),
        }
    }
    pub fn with_dimensions(&mut self, width: u32, height: u32) -> &mut Self {
        self.config.dimensions = (width, height);
        self
    }

    pub fn with_ship_dimensions(&mut self, width: u32, height: u32) -> &mut Self {
        self.config.ship_dimensions = (width, height);
        self
    }

    pub fn with_ship_speed(&mut self, speed: u32) -> &mut Self {
        self.config.ship_speed = speed;
        self
    }

    pub fn with_bullet_dimensions(&mut self, width: u32, height: u32) -> &mut Self {
        self.config.bullet_dimensions = (width, height);
        self
    }


    pub fn with_bullet_speed(&mut self, speed: u32) -> &mut Self {
        self.config.bullet_speed = speed;
        self
    }

    pub fn with_input(&mut self, input: InputRef) -> &mut Self {
        self.input = input;
        self
    }

    pub fn build(&self) -> Game {
        let mut world = World::new();
        world.register::<Geometry>();
        world.register::<Velocity>();
        world.register::<IsPlayer>();
        world.register::<Sprite>();

        let (ship_width, ship_height) = self.config.ship_dimensions;
        let (width, height) = self.config.dimensions;
        let x = width / 2 - ship_width / 2;
        let y = height - ship_height;
        world.create_entity()
            .with(Geometry { x: x as i32, y: y as i32, width: ship_width, height: ship_height })
            .with(Velocity(0, 0))
            .with(Sprite { id: 0 })
            .with(IsPlayer)
            .build();

        world.insert(self.config.clone());

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
        self.handle_player_input();
        self.move_game_objects();
        self.constrain_player_to_screen();
        self.world.maintain();
    }


    fn handle_player_input(&mut self) {
        let mut velocities = self.world.write_storage::<Velocity>();
        let gos = self.world.read_storage::<Geometry>();
        let players = self.world.read_storage::<IsPlayer>();
        let config = self.world.read_resource::<GameConfig>();
        let updater = self.world.read_resource::<LazyUpdate>();
        let entities = self.world.entities();


        for (velocity, player_geom, _) in (&mut velocities, &gos, &players).join() {
            if self.input.borrow().is_key_down(KEY_SPACE) {
                let bullet = entities.create();
                let x = player_geom.x as u32 + config.ship_dimensions.0 / 2 - config.bullet_dimensions.0 / 2;
                let y = player_geom.y as u32 - config.bullet_dimensions.1;
                updater.insert(bullet, Geometry {
                    x: x as i32,
                    y: y as i32,
                    width: config.bullet_dimensions.0,
                    height: config.bullet_dimensions.1,
                });
                updater.insert(bullet, Velocity(0, -1 * config.bullet_speed as i32));
                updater.insert(bullet, Sprite { id: 1 });
            }

            if self.input.borrow().is_key_down(KEY_LEFT) {
                velocity.0 = config.ship_speed as i32 * -1;
            } else if self.input.borrow().is_key_down(KEY_RIGHT) {
                velocity.0 = config.ship_speed as i32;
            } else {
                velocity.0 = 0;
            }
        }
    }

    fn move_game_objects(&mut self) {
        let mut gos = self.world.write_storage::<Geometry>();
        let velocities = self.world.read_storage::<Velocity>();

        for (geometry, velocity) in (&mut gos, &velocities).join() {
            geometry.x += velocity.0;
            geometry.y += velocity.1;
        }
    }

    fn constrain_player_to_screen(&mut self) {
        let config = self.world.read_resource::<GameConfig>();
        let mut gos = self.world.write_storage::<Geometry>();
        let players = self.world.read_storage::<IsPlayer>();


        for (geometry, _) in (&mut gos, &players).join() {
            let max_x = (config.dimensions.0 - geometry.width) as i32;
            geometry.x = max(0, min(geometry.x, max_x));
        }
    }


    pub fn render<T>(&self, renderer: &mut T) where T: GameRenderer {
        renderer.clear();
        let gos = self.world.read_storage::<Geometry>();
        let sprites = self.world.read_storage::<Sprite>();
        for (geometry, sprite) in (&gos, &sprites).join() {
            renderer.draw_sprite(sprite.id, geometry.x, geometry.y, geometry.width, geometry.height);
        }
    }
}