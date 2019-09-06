use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::game::input::Input;

pub type InputStubRef = Rc<RefCell<InputStub>>;

pub struct InputStub {
    key_states: HashMap<u32, bool>
}

impl InputStub {
    pub fn new_rc() -> InputStubRef {
        Rc::new(RefCell::new(Self {
            key_states: HashMap::new()
        }))
    }

    pub fn key_is_down(&mut self, key: u32) {
        self.key_states.insert(key, true);
    }

    pub fn key_is_up(&mut self, key: u32) {
        self.key_states.insert(key, false);
    }

}

impl Input for InputStub {
    fn is_key_down(&self, key: u32) -> bool {
        *self.key_states.get(&key).unwrap_or(&false)
    }
}