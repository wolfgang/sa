use raylib::consts::*;
use specs::prelude::*;

use builder::GameBuilder;
use components::*;
use input::InputRef;

use crate::gfx::game_renderer::GameRenderer;

pub mod builder;
pub mod input;
pub mod components;
pub mod systems;
pub mod ecs;

#[derive(Default)]
pub struct GameState {
    current_tick: u32,
    last_bullet_tick: u32,
    moving_left: bool,
    moving_right: bool,
    shooting: bool,
    active_enemies: u32
}

pub struct Game {
    world: World,
    input: InputRef,
    dispatcher: Dispatcher<'static, 'static>
}

impl Game {
    pub fn init() -> GameBuilder {
        GameBuilder::new()
    }

    pub fn new(world: World, dispatcher: Dispatcher<'static, 'static>, input: InputRef) -> Self {
        Self {
            world,
            input,
            dispatcher
        }
    }

    pub fn tick(&mut self) {
        self.update_game_state();
        self.dispatcher.dispatch(&self.world);
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