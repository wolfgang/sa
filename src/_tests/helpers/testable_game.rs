use crate::_tests::helpers::input_stub::{InputStub, InputStubRef};
use crate::_tests::helpers::string_renderer::StringRenderer;
use crate::game::{builder::GameBuilder, Game};

const DEFAULT_FPS: u32 = 60;

const DEFAULT_SHIP_WIDTH: u32 = 4;
const DEFAULT_SHIP_HEIGHT: u32 = 1;
const DEFAULT_SHIP_SPEED: u32 = 120;

const DEFAULT_BULLET_WIDTH: u32 = 2;
const DEFAULT_BULLET_HEIGHT: u32 = 1;
const DEFAULT_BULLET_SPEED: u32 = 60;

const DEFAULT_ENEMY_WIDTH: u8 = 2;
const DEFAULT_ENEMY_HEIGHT: u8 = 2;

pub struct TestableGameBuilder {
    game_builder: GameBuilder
}

impl TestableGameBuilder {
    pub fn new() -> Self {
        Self {
            game_builder: GameBuilder::new()
                .with_fps(DEFAULT_FPS)
                .with_ship_speed(DEFAULT_SHIP_SPEED)
                .with_ship_dimensions(DEFAULT_SHIP_WIDTH, DEFAULT_SHIP_HEIGHT)
                .with_bullet_dimensions(DEFAULT_BULLET_WIDTH, DEFAULT_BULLET_HEIGHT)
                .with_bullet_speed(DEFAULT_BULLET_SPEED)
                .clone()
        }
    }

    pub fn with_dimensions(&mut self, width: u32, height: u32) -> &mut Self {
        self.game_builder = self.game_builder.with_dimensions(width, height).clone();
        self
    }

    pub fn with_enemy_speed(&mut self, dx: u32, dy: u32) -> &mut Self {
        self.game_builder = self.game_builder.with_enemy_speed(dx, dy).clone();
        self
    }

    pub fn build(&mut self) -> TestableGame {
        let input = InputStub::new_rc();
        let game = self.game_builder
            .with_input(input.clone())
            .build();

        let (width, height) = self.game_builder.dimensions;
        let mut renderer = StringRenderer::new(width as usize, height as usize);
        renderer.register_sprite(0, DEFAULT_SHIP_WIDTH as u8, DEFAULT_SHIP_HEIGHT as u8);
        renderer.register_sprite(1, DEFAULT_BULLET_WIDTH as u8, DEFAULT_BULLET_HEIGHT as u8);
        renderer.register_sprite(2, DEFAULT_ENEMY_WIDTH, DEFAULT_ENEMY_HEIGHT);
        TestableGame {
            input: input.clone(),
            renderer,
            game,
        }
    }
}

pub struct TestableGame {
    input: InputStubRef,
    renderer: StringRenderer,
    game: Game,
}

impl TestableGame {
    pub fn init() -> TestableGameBuilder {
        TestableGameBuilder::new()
    }

    pub fn key_is_down(&mut self, key: u32) {
        self.input.borrow_mut().key_is_down(key)
    }

    pub fn key_is_up(&mut self, key: u32) {
        self.input.borrow_mut().key_is_up(key)
    }

    pub fn tick_twice(&mut self) {
        self.tick_times(2);
    }

    pub fn tick_times(&mut self, times: u32) {
        for _ in 0..times { self.tick() }
    }

    pub fn loop_times(&mut self, times: u32) {
        for _ in 0..times {
            self.tick();
            self.render();
        }
    }

    pub fn tick(&mut self) {
        self.game.tick();
    }

    pub fn render(&mut self) {
        self.game.render(&mut self.renderer);
    }

    pub fn renders_frame(&mut self, expected_frame: Vec<&str>) {
        self.render();
        self.assert_frame(expected_frame);
    }

    pub fn assert_frame(&self, expected_frame: Vec<&str>) {
        self.renderer.assert_frame(expected_frame)
    }

    pub fn assert_sprite_log_for(&self, id: u8, expected_log: Vec<&str>) {
        self.renderer.assert_sprite_log_for(id, expected_log)
    }
}