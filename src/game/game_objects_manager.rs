use crate::game::game_object::GameObjectRef;
use crate::game::renderer::GameRenderer;

pub struct GameObjectsManager {
    pub game_objects: Vec<GameObjectRef>
}

impl GameObjectsManager {
    pub fn new() -> Self {
        Self {
            game_objects: Vec::with_capacity(10)
        }
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

