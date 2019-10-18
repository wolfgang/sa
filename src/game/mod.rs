use specs::prelude::*;

use builder::GameBuilder;
use components::*;
use input::InputRef;
use systems::*;

use crate::gfx::game_renderer::GameRenderer;

pub mod builder;
pub mod input;
pub mod components;
pub mod systems;

#[derive(Default)]
pub struct GameState {
    current_tick: u32,
    last_bullet_tick: u32,
}

pub struct Game {
    world: World,
    handle_player_input: HandlePlayerInput,
    move_game_objects: MoveGameObjects,
    constrain_player_to_screen: ConstrainPlayerToScreen
}

impl Game {
    pub fn init() -> GameBuilder {
        GameBuilder::new()
    }

    pub fn new(world: World, input: InputRef) -> Self {
        Self {
            world,
            handle_player_input: HandlePlayerInput::new(input),
            move_game_objects: MoveGameObjects {},
            constrain_player_to_screen: ConstrainPlayerToScreen {},
        }
    }

    pub fn tick(&mut self) {
        self.handle_player_input.run_now(&self.world);
        self.move_game_objects.run_now(&self.world);
        self.constrain_player_to_screen.run_now(&self.world);

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