use std::cell::RefCell;
use std::rc::Rc;

use crate::game::builder::GameBuilder;
use crate::game::objects::enemy_ship::EnemyShip;
use crate::game::objects::game_object::{GameObjectRef, NullGameObject};
use crate::game::objects::player_ship::{PlayerShip, PlayerShipRef};
use crate::gfx::renderer::GameRenderer;

pub type GameObjectsManagerRef = Rc<RefCell<GameObjectsManager>>;

pub struct GameObjectsManager {
    pub game_objects: Vec<GameObjectRef>,
    player_ship: PlayerShipRef,
    enemy_ship: GameObjectRef,
    game_builder: GameBuilder
}

impl GameObjectsManager {
    pub fn new_rc(game_builder: &GameBuilder) -> GameObjectsManagerRef {
        let player_ship = PlayerShip::from_game_builder(game_builder);
        let enemy_ship = Self::new_enemy_ship(game_builder);
        Rc::new(RefCell::new(
            Self {
                game_objects: vec![player_ship.clone() as GameObjectRef, enemy_ship.clone()],
                player_ship: player_ship.clone().clone(),
                enemy_ship,
                game_builder: game_builder.clone()
            }))
    }

    pub fn player_ship(&self) -> PlayerShipRef {
        self.player_ship.clone()
    }

    pub fn add(&mut self, game_object: GameObjectRef) {
        self.game_objects.push(game_object);
    }

    pub fn tick(&mut self) {
        self.tick_game_objects();
        self.resolve_collisions();
        if !self.enemy_ship.borrow().is_alive() {
            self.enemy_ship = Self::new_enemy_ship(&self.game_builder);
            self.add(self.enemy_ship.clone())
        }
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

    fn new_enemy_ship(game_builder: &GameBuilder) -> GameObjectRef {
        let enemies_enabled = game_builder.enemy_speed().x > 0.0;
        if enemies_enabled { EnemyShip::from_game_builder(game_builder) } else { NullGameObject::new_rc() }
    }
}
