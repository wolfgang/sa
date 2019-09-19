use std::cell::RefCell;
use std::rc::Rc;

use crate::game::builder::GameBuilder;
use crate::game::objects::enemy_ship::EnemyShip;
use crate::game::objects::game_object::{GameObjectRef, NullGameObject};
use crate::game::objects::player_ship::PlayerShip;
use crate::gfx::renderer::GameRenderer;

pub type GameObjectsManagerRef = Rc<RefCell<GameObjectsManager>>;

pub struct GameObjectsManager {
    pub game_objects: Vec<GameObjectRef>
}

impl GameObjectsManager {
    pub fn new_rc() -> GameObjectsManagerRef {
        Rc::new(RefCell::new(
            Self {
                game_objects: Vec::with_capacity(10)
            }))
    }

    pub fn spawn_initial_objects(&mut self, game_builder: &GameBuilder) {
        let player_ship = PlayerShip::from_game_builder(game_builder);
        let enemies_enabled = game_builder.enemy_speed().x > 0.0;
        let enemy_ship = if enemies_enabled { EnemyShip::from_game_builder(game_builder) } else { NullGameObject::new_rc() };

        self.add(player_ship.clone() as GameObjectRef);
        self.add(enemy_ship);
    }

    pub fn add(&mut self, game_object: GameObjectRef) {
        self.game_objects.push(game_object);
    }

    pub fn tick(&mut self) {
        self.tick_game_objects();
        self.resolve_collisions();
        self.remove_dead_game_objects();
    }

    pub fn render<T>(&self, renderer: &mut T) where T: GameRenderer {
        for go in self.game_objects.iter() {
            go.borrow().render(renderer);
        }
    }

    fn tick_game_objects(&mut self) {
        for go in self.game_objects.iter() {
            go.borrow_mut().tick();
        }
    }

    fn resolve_collisions(&mut self) {
        for i in 0..self.game_objects.len() - 1 {
            let go1 = &self.game_objects[i];
            for j in i + 1..self.game_objects.len() {
                let go2 = &self.game_objects[j];
                if go1.borrow().is_colliding_with(go2) {
                    go2.borrow_mut().on_collision();
                    go1.borrow_mut().on_collision();
                }
            }
        }
    }

    fn remove_dead_game_objects(&mut self) {
        self.game_objects.retain(|go| { go.borrow().is_alive() });
    }
}
