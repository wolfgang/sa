use crate::gfx::game_renderer::GameRenderer;

pub struct GameBuilder {
    dimensions: (u32, u32),
    ship_dimensions: (u32, u32),
}

impl GameBuilder {
    pub fn with_dimensions(&mut self, width: u32, height: u32) -> &mut Self {
        self.dimensions = (width, height);
        self
    }

    pub fn with_ship_dimensions(&mut self, width: u32, height: u32) -> &mut Self {
        self.ship_dimensions = (width, height);
        self
    }

    pub fn build(&self) -> Game {
        Game {
            dimensions: self.dimensions,
            ship_dimensions: self.ship_dimensions,
        }
    }
}

pub struct Game {
    pub dimensions: (u32, u32),
    pub ship_dimensions: (u32, u32),
}

impl Game {
    pub fn init() -> GameBuilder {
        GameBuilder {
            dimensions: (0, 0),
            ship_dimensions: (0, 0),
        }
    }

    pub fn render(&self, renderer: &mut dyn GameRenderer) {
        let x = self.dimensions.0 / 2 - self.ship_dimensions.0 / 2;
        renderer.draw_sprite(0, x, self.dimensions.1 - self.ship_dimensions.1);
    }
}