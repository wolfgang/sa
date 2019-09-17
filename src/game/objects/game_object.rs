use std::cell::RefCell;
use std::rc::Rc;

use crate::game::renderer::GameRenderer;

pub type GameObjectRef = Rc<RefCell<dyn GameObject>>;

pub trait GameObject {
    fn tick(&mut self);
    fn render(&self, renderer: &mut dyn GameRenderer);
    fn is_alive(&self) -> bool { true }
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