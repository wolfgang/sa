use std::cell::RefCell;
use std::rc::Rc;

use crate::game::builder::GameBuilder;
use crate::game::game_object::GameObject;
use crate::game::geometry::{Rectanglef, Vector2, Vector2f};
use crate::game::moving_sprite::MovingSprite;
use crate::game::renderer::GameRenderer;

pub type PlayerShipRef = Rc<RefCell<PlayerShip>>;

pub struct PlayerShip {
    width: u32,
    screen_rect: Rectanglef,
    game_object: MovingSprite,
}

impl PlayerShip {
    pub fn from_game_builder(builder: &GameBuilder) -> PlayerShipRef {
        let (width, height) = builder.ship_dimensions;
        let (game_width, game_height) = builder.dimensions;
        let ship_x = game_width / 2 - width / 2;
        let ship_y = game_height - height;

        let go_rect = Rectanglef::with_tuple(Vector2::with(ship_x as f32, ship_y as f32), builder.ship_dimensions);
        Rc::new(RefCell::new(Self {
            width,
            game_object: MovingSprite::new(0, go_rect, builder.ship_speed()),
            screen_rect: builder.screen_rect(),
        }))
    }

    pub fn render<T>(&self, renderer: &mut T) where T: GameRenderer {
        renderer.draw_sprite_obj(&self.game_object)
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
        if !self.game_object.is_inside_of(&self.screen_rect) {
            self.game_object.snap_to(&self.screen_rect);
        }
    }

    pub fn bullet_spawn_position(&self) -> Vector2f {
        let pos = self.game_object.get_position();
        Vector2::with(pos.x + self.width as f32 / 2.0, pos.y)
    }
}

impl GameObject for PlayerShip {
    fn tick(&mut self) {
        self.game_object.tick();
        if !self.game_object.is_inside_of(&self.screen_rect) {
            self.game_object.snap_to(&self.screen_rect);
        }
    }

    fn render(&self, renderer: &mut dyn GameRenderer) {
        renderer.draw_sprite_obj(&self.game_object)
    }
}
