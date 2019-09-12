use crate::game::builder::GameBuilder;
use crate::game::game_object::GameObject;
use crate::game::geometry::Vector2;
use crate::game::renderer::GameRenderer;

pub struct PlayerShip {
    width: u32,
    bullet_dimensions: Vector2<u32>,
    max_x: u32,
    game_object: GameObject
}

impl PlayerShip {
    pub fn from_game_builder(builder: &GameBuilder) -> PlayerShip {
        let (width, height) = builder.ship_dimensions;
        let (game_width, game_height) = builder.dimensions;
        let ship_x = game_width / 2 - width / 2;
        let ship_y = game_height - height;

        let game_object = GameObject::new(0, ship_x as i32, ship_y as i32, (builder.ship_speed(), 0));

        Self {
            width,
            bullet_dimensions: Vector2::from(builder.bullet_dimensions),
            max_x: game_width - width,
            game_object
        }
    }

    pub fn render<T>(&self, renderer: &mut T) where T: GameRenderer {
        self.game_object.render(renderer)
    }

    pub fn move_left(&mut self) {
        self.game_object.set_move_direction(-1, 0);
    }

    pub fn move_right(&mut self) {
        self.game_object.set_move_direction(1, 0);
    }

    pub fn stop(&mut self) {
        self.game_object.set_move_direction(0, 0);
    }

    pub fn tick(&mut self) {
        self.game_object.tick();
        let (x, y) = self.game_object.get_position();

        if x < 0 { self.game_object.set_position(0, y) };
        if x >= self.max_x as i32 { self.game_object.set_position(self.max_x as i32, y) }
    }

    pub fn bullet_spawn_position(&self) -> (i32, i32) {
        let (bullet_width, bullet_height) = self.bullet_dimensions.into();
        let (x, y) = self.game_object.get_position();
        (x + self.width as i32 / 2 - bullet_width as i32 / 2, y - bullet_height as i32)
    }
}

