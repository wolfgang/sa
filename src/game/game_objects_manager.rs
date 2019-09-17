use std::cell::RefCell;
use std::rc::Rc;

use crate::game::objects::game_object::GameObjectRef;
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

    pub fn add(&mut self, game_object: GameObjectRef) {
        self.game_objects.push(game_object);
    }

    pub fn tick(&mut self) {
        for go in self.game_objects.iter() {
            go.borrow_mut().tick();
        }
        self.game_objects.retain(|go| { go.borrow().is_alive() });
    }

    pub fn render<T>(&self, renderer: &mut T) where T: GameRenderer {
        for go in self.game_objects.iter() {
            go.borrow().render(renderer);
        }
    }
}
