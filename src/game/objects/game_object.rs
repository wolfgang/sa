use std::cell::RefCell;
use std::rc::Rc;

use crate::game::objects::moving_sprite::MovingSprite;
use crate::gfx::renderer::GameRenderer;

pub type GameObjectRef = Rc<RefCell<dyn GameObject>>;

pub trait GameObject {
    fn tick(&mut self);
    fn render(&self, renderer: &mut dyn GameRenderer);
    fn is_alive(&self) -> bool { true }
    fn is_target(&self) -> bool { false }
    fn process_collision(&mut self, _: &MovingSprite) -> bool {
        false
    }

    fn check_collisions(&mut self, _: &Vec<GameObjectRef>) {}
}

pub struct NullGameObject {}

impl NullGameObject {
    pub fn new_rc() -> GameObjectRef {
        Rc::new(RefCell::new(Self {}))
    }
}

impl GameObject for NullGameObject {
    fn tick(&mut self) {}
    fn render(&self, _renderer: &mut dyn GameRenderer) {}
}