use crate::game::renderer::GameRenderer;

pub trait GameObject {
    fn tick(&mut self);
    fn render<T>(&self, renderer: &mut T) where T: GameRenderer, Self: Sized;
    fn is_alive(&self) -> bool { true }
}