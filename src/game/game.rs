use crate::gfx::game_renderer::GameRenderer;

pub struct GameBuilder {}

impl GameBuilder {
    pub fn with_dimensions(&mut self, width: u32, height: u32) -> &mut Self {
        self
    }

    pub fn with_ship_dimensions(&mut self, width: u32, height: u32) -> &mut Self {
        self
    }

    pub fn build(&self) -> Game {
        Game {}
    }
}

pub struct Game {}

impl Game {
    pub fn init() -> GameBuilder {
        GameBuilder {}
    }

    pub fn render<T>(&self, renderer: &mut T) where T: GameRenderer {
        renderer.draw_sprite(0, 3, 4, 3, 1);
    }
}