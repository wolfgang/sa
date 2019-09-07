use std::cell::RefCell;
use std::rc::Rc;

use raylib::consts::*;

use builder::GameBuilder;
use input::InputRef;
use player_ship::PlayerShip;
use renderer::GameRenderer;

use crate::game::bullets_manager::BulletsManager;

pub mod builder;
pub mod input;
pub mod renderer;
pub mod positioned;
pub mod player_ship;
pub mod player_bullet;
pub mod bullets_manager;

pub struct Game {
    input: InputRef,
    fps: u32,
    ship_speed: u32,
    player_ship: Rc<RefCell<PlayerShip>>,
    bullets_manager: BulletsManager
}

impl Game {
    pub fn init() -> GameBuilder {
        GameBuilder::new()
    }

    pub fn from_builder(builder: &GameBuilder) -> Self {
        let player_ship = Rc::new(RefCell::new(PlayerShip::new(builder.ship_dimensions, builder.dimensions)));
        Game {
            input: builder.input.clone(),
            fps: builder.fps,
            player_ship: player_ship.clone(),
            ship_speed: builder.ship_speed,
            bullets_manager: BulletsManager::new(player_ship.clone(), builder.fps, builder.bullet_dimensions, builder.bullet_speed)
        }
    }

    pub fn tick(&mut self) {
        let offset = (self.ship_speed / self.fps) as i32;
        if self.input.borrow().is_key_down(KEY_LEFT) {
            self.player_ship.borrow_mut().move_horizontally(-1 * offset);
        }

        if self.input.borrow().is_key_down(KEY_RIGHT) {
            self.player_ship.borrow_mut().move_horizontally(offset);
        }

        if self.input.borrow().is_key_down(KEY_SPACE) {
            self.bullets_manager.spawn_bullet();
        } else {
            self.bullets_manager.reset();
        }
        self.bullets_manager.tick();
    }

    pub fn render(&self, renderer: &mut dyn GameRenderer) {
        renderer.clear();
        self.player_ship.borrow_mut().render(renderer);
        self.bullets_manager.render_bullets(renderer);
    }
}