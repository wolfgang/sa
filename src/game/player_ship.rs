use crate::game::builder::GameBuilder;
use crate::game::positioned::Positioned;
use crate::game::renderer::GameRenderer;

pub struct PlayerShip {
    x: i32,
    y: i32,
    width: u32,
    bullet_dimensions: (u32, u32),
    speed: u32,
    move_direction: i32,
    max_x: u32,
}

impl PlayerShip {
    pub fn from_game_builder(builder: &GameBuilder) -> PlayerShip {
        let (width, height) = builder.ship_dimensions;
        let (game_width, game_height) = builder.dimensions;
        let ship_x = game_width / 2 - width / 2;
        let ship_y = game_height - height;

        Self {
            x: ship_x as i32,
            y: ship_y as i32,
            width,
            bullet_dimensions: builder.bullet_dimensions,
            speed: builder.ship_speed(),
            move_direction: 0,
            max_x: game_width - width,
        }
    }

    pub fn render<T>(&self, renderer: &mut T) where T: GameRenderer {
        renderer.draw_sprite_obj(0, self)
    }

    pub fn move_left(&mut self) {
        self.move_direction = -1
    }

    pub fn move_right(&mut self) {
        self.move_direction = 1;
    }

    pub fn stop(&mut self) {
        self.move_direction = 0;
    }

    pub fn tick(&mut self) {
        self.x += self.move_direction * self.speed as i32;
        if self.x < 0 { self.x = 0 };
        if self.x >= self.max_x as i32 { self.x = self.max_x as i32 }
    }

    pub fn bullet_spawn_position(&self) -> (i32, i32) {
        let (bullet_width, bullet_height) = self.bullet_dimensions;
        (self.x + self.width as i32 / 2 - bullet_width as i32 / 2, self.y - bullet_height as i32)
    }
}

impl Positioned for PlayerShip {
    fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }
}