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
    player_ship: Rc<RefCell<PlayerShip>>,
    bullets_manager: BulletsManager,
}

impl Game {
    pub fn init() -> GameBuilder {
        GameBuilder::new()
    }

    pub fn from_builder(builder: &GameBuilder) -> Self {
        let player_ship = Rc::new(RefCell::new(PlayerShip::new(builder.ship_dimensions, builder.dimensions, builder.ship_speed())));
        Game {
            input: builder.input.clone(),
            player_ship: player_ship.clone(),
            bullets_manager: BulletsManager::new(player_ship.clone(), builder.bullet_interval(), builder.bullet_dimensions, builder.bullet_speed()),
        }
    }

    pub fn tick(&mut self) {
        if self.input.borrow().is_key_down(KEY_LEFT) {
            self.player_ship.borrow_mut().move_left();
        } else if self.input.borrow().is_key_down(KEY_RIGHT) {
            self.player_ship.borrow_mut().move_right();
        } else { self.player_ship.borrow_mut().stop() }

        if self.input.borrow().is_key_down(KEY_SPACE) {
            self.bullets_manager.spawn_bullet();
        } else {
            self.bullets_manager.reset();
        }
        self.player_ship.borrow_mut().tick();
        self.bullets_manager.tick();
    }

    pub fn render(&self, renderer: &mut dyn GameRenderer) {
        renderer.clear();
        self.player_ship.borrow_mut().render(renderer);
        self.bullets_manager.render_bullets(renderer);
    }
}