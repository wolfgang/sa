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
    fn is_colliding_with(&self, other: &GameObjectRef) -> bool {
        self.collider().intersects_with(other.borrow().collider())
    }

    fn on_collision(&mut self) {}

    fn collider(&self) -> &MovingSprite;
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