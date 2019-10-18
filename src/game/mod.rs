use std::cmp::{max, min};

use raylib::consts::{KEY_LEFT, KEY_RIGHT, KEY_SPACE};
use specs::prelude::*;

use builder::{GameBuilder, GameConfig};
use components::*;
use input::InputRef;

use crate::gfx::game_renderer::GameRenderer;

pub mod builder;
pub mod input;
pub mod components;


struct MoveGameObjects {}

impl<'a> System<'a> for MoveGameObjects {
    type SystemData = (
        WriteStorage<'a, Geometry>,
        ReadStorage<'a, Velocity>
    );

    fn run(&mut self, (mut geometries, velocities): Self::SystemData) {
        for (geometry, velocity) in (&mut geometries, &velocities).join() {
            geometry.x += velocity.0;
            geometry.y += velocity.1;
        }
    }
}


#[derive(Default)]
struct GameState {
    current_tick: u32,
    last_bullet_tick: u32,
}

pub struct Game {
    world: World,
    input: InputRef,
}

impl Game {
    pub fn init() -> GameBuilder {
        GameBuilder::new()
    }

    pub fn new(world: World, input: InputRef) -> Self {
        Self { world, input }
    }

    pub fn tick(&mut self) {
        self.handle_player_input();

        let mut sys = MoveGameObjects {};
        sys.run_now(&self.world);
        self.constrain_player_to_screen();
        {
            let mut game_state = self.world.write_resource::<GameState>();
            game_state.current_tick += 1;
        }
        self.world.maintain();
    }


    fn handle_player_input(&mut self) {
        let mut velocities = self.world.write_storage::<Velocity>();
        let gos = self.world.read_storage::<Geometry>();
        let players = self.world.read_storage::<IsPlayer>();
        let config = self.world.read_resource::<GameConfig>();
        let updater = self.world.read_resource::<LazyUpdate>();
        let entities = self.world.entities();
        let mut game_state = self.world.write_resource::<GameState>();


        for (velocity, player_geom, _) in (&mut velocities, &gos, &players).join() {
            if self.input.borrow().is_key_down(KEY_SPACE) {
                if game_state.current_tick - game_state.last_bullet_tick >= config.autofire_delay {
                    game_state.last_bullet_tick = game_state.current_tick;
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
            } else {
                game_state.last_bullet_tick = 0;
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