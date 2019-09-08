use std::cell::RefCell;
use std::rc::Rc;

use crate::game::builder::GameBuilder;
use crate::game::positioned::Positioned;
use crate::game::renderer::GameRenderer;

pub type PlayerShipRef = Rc<RefCell<PlayerShip>>;

pub struct PlayerShip {
    x: i32,
    y: i32,
    width: u32,
    speed: u32,
    current_speed: i32,
    max_x: u32,
}

impl PlayerShip {
    pub fn from_game_builder_rc(builder: &GameBuilder) -> PlayerShipRef {
        let (width, height) = builder.ship_dimensions;
        let (game_width, game_height) = builder.dimensions;
        let ship_x = game_width / 2 - width / 2;
        let ship_y = game_height - height;

        Rc::new(RefCell::new(
            Self {
                x: ship_x as i32,
                y: ship_y as i32,
                width,
                speed: builder.ship_speed(),
                current_speed: 0,
                max_x: game_width - width,
            }
        ))
    }

    pub fn render(&self, renderer: &mut dyn GameRenderer) {
        renderer.draw_sprite_obj(0, self)
    }

    pub fn move_left(&mut self) {
        self.current_speed = -1 * (self.speed as i32);
    }

    pub fn move_right(&mut self) {
        self.current_speed = self.speed as i32;
    }

    pub fn stop(&mut self) {
        self.current_speed = 0;
    }


    pub fn tick(&mut self) {
        self.x += self.current_speed;
        if self.x < 0 { self.x = 0 };
        if self.x >= self.max_x as i32 { self.x = self.max_x as i32 }
    }

    pub fn bullet_spawn_position(&self, bullet_width: i32, bullet_height: i32) -> (i32, i32) {
        return (self.x + self.width as i32 / 2 - bullet_width / 2, self.y - bullet_height);
    }
}

impl Positioned for PlayerShip {
    fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }
}