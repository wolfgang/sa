use std::cell::RefCell;
use std::rc::Rc;

use crate::game::objects::moving_sprite::MovingSprite;
use crate::gfx::renderer::GameRenderer;

pub type GameObjectRef = Rc<RefCell<dyn GameObject>>;

pub trait GameObject {
    fn tick(&mut self);
    fn render(&self, renderer: &mut dyn GameRenderer);
    fn collider(&self) -> &MovingSprite;

    fn is_alive(&self) -> bool { true }
    fn on_collision(&mut self) {}
    fn is_colliding_with(&self, other: &GameObjectRef) -> bool {
        self.collider().intersects_with(other.borrow().collider())
    }
}

pub struct NullGameObject {
    moving_sprite: MovingSprite
}

impl NullGameObject {
    pub fn new_rc() -> GameObjectRef {
        Rc::new(RefCell::new(Self { moving_sprite: MovingSprite::default() }))
    }
}

impl GameObject for NullGameObject {
    fn tick(&mut self) {}
    fn render(&self, _renderer: &mut dyn GameRenderer) {}

    fn collider(&self) -> &MovingSprite {
        &self.moving_sprite
    }
}