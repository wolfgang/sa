use std::cell::RefCell;
use std::rc::Rc;

use crate::game::input::Input;

type InputStubRef = Rc<RefCell<InputStub>>;

pub struct InputStub {}

impl InputStub {
    pub fn new_rc() -> InputStubRef {
        Rc::new(RefCell::new(Self {}))
    }

    pub fn key_is_down(&mut self, _key: u32) {}
}

impl Input for InputStub {
    fn is_key_down(&self, _key: u32) -> bool {
        false
    }
}