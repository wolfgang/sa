use raylib::consts::*;
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
    moving_left: bool,
    moving_right: bool,
    shooting: bool
}

pub struct Game {
    world: World,
    input: InputRef,
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
            input,
            handle_player_input: HandlePlayerInput {},
            move_game_objects: MoveGameObjects {},
            constrain_player_to_screen: ConstrainPlayerToScreen {},
        }
    }

    pub fn tick(&mut self) {
        self.update_game_state();
        self.handle_player_input.run_now(&self.world);
        self.move_game_objects.run_now(&self.world);
        self.constrain_player_to_screen.run_now(&self.world);

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

    fn update_game_state(&mut self) {
        let mut game_state = self.world.write_resource::<GameState>();
        game_state.current_tick += 1;
        game_state.moving_left = self.input.borrow().is_key_down(KEY_LEFT);
        game_state.moving_right = self.input.borrow().is_key_down(KEY_RIGHT);
        game_state.shooting = self.input.borrow().is_key_down(KEY_SPACE);
    }
}