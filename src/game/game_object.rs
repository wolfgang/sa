use std::cell::RefCell;
use std::rc::Rc;

use crate::game::renderer::GameRenderer;

pub type GameObjectRef = Rc<RefCell<dyn GameObject>>;

pub trait GameObject {
    fn tick(&mut self);
    fn render(&self, renderer: &mut dyn GameRenderer);
    fn is_alive(&self) -> bool { true }
}