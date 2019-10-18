use std::cmp::{max, min};

use specs::prelude::*;

use builder::{GameBuilder, GameConfig};
use components::*;
use input::InputRef;
use systems::handle_player_input::HandlePlayerInput;

use crate::gfx::game_renderer::GameRenderer;

pub mod builder;
pub mod input;
pub mod components;
pub mod systems;


struct MoveGameObjects {}
struct ConstrainPlayerToScreen {}


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

impl<'a> System<'a> for ConstrainPlayerToScreen {
    type SystemData = (
        WriteStorage<'a, Geometry>,
        ReadStorage<'a, IsPlayer>,
        Read<'a, GameConfig>
    );

    fn run(&mut self, (mut gos, players, config): Self::SystemData) {
        for (geometry, _) in (&mut gos, &players).join() {
            let max_x = (config.dimensions.0 - geometry.width) as i32;
            geometry.x = max(0, min(geometry.x, max_x));
        }
    }
}


#[derive(Default)]
pub struct GameState {
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
        let mut handle_player_input = HandlePlayerInput::new(self.input.clone());
        let mut move_game_objects = MoveGameObjects {};
        let mut constrain_player_to_screen = ConstrainPlayerToScreen {};
        handle_player_input.run_now(&self.world);
        move_game_objects.run_now(&self.world);
        constrain_player_to_screen.run_now(&self.world);

        self.advance_tick();
        self.world.maintain();
    }

    pub fn render<T>(&self, renderer: &mut T) where T: GameRenderer {
        renderer.clear();
        let gos = self.world.read_storage::<Geometry>();
        let sprites = self.world.read_storage::<Sprite>();
        for (geometry, sprite) in (&gos, &sprites).join() {
            renderer.draw_sprite(sprite.id, geometry.x, geometry.y, geometry.width, geometry.height);
        }
    }

    fn advance_tick(&mut self) {
        let mut game_state = self.world.write_resource::<GameState>();
        game_state.current_tick += 1;
    }
}