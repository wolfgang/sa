use std::cell::RefCell;
use std::rc::Rc;

use crate::core::geometry::Rectanglef;
use crate::gfx::renderer::GameRenderer;

pub type GameObjectRef = Rc<RefCell<dyn GameObject>>;

pub trait GameObject {
    fn tick(&mut self);
    fn render(&self, renderer: &mut dyn GameRenderer);
    fn is_alive(&self) -> bool { true }
    fn process_collision(&mut self, _: &Rectanglef) -> bool {
        false
    }
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