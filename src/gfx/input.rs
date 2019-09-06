use std::cell::RefCell;
use std::rc::Rc;

use raylib::RaylibHandle;

use crate::game::input::{Input, InputRef};

pub struct RaylibInput {
    rl: Rc<RaylibHandle>
}

impl RaylibInput {
    pub fn new_rc(rl: Rc<RaylibHandle>) -> InputRef {
        Rc::new(RefCell::new(Self { rl }))
    }
}

impl Input for RaylibInput {
    fn is_key_down(&self, key: u32) -> bool {
        self.rl.is_key_down(key as i32)
    }
}