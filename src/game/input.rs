use std::cell::RefCell;
use std::rc::Rc;

pub type InputRef = Rc<RefCell<dyn Input>>;

pub trait Input {
    fn is_key_down(&self, key: u32) -> bool;
}

pub struct InputNull {}

impl InputNull {
    pub fn new_rc() -> InputRef {
        Rc::new(RefCell::new(Self {}))
    }
}

impl Input for InputNull {
    fn is_key_down(&self, _key: u32) -> bool {
        false
    }
}