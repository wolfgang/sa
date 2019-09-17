use std::cell::{Ref, RefCell};
use std::rc::Rc;

use crate::_tests::helpers::input_stub::InputStub;
use crate::game::input::{Input, InputRef};

type ComponentRef<T> = Rc<RefCell<T>>;

trait Component {
    fn tick(&mut self, entity: &GameEntity);
}

#[derive(Default)]
struct TestComponent1 {
    spy: TickSpyRef
}

impl TestComponent1 {
    fn new_rc(spy: TickSpyRef) -> ComponentRef<Self> {
        Rc::new(RefCell::new(Self { spy }))
    }
}

impl Component for TestComponent1 {
    fn tick(&mut self, entity: &GameEntity) {
        self.spy.borrow_mut().log("TestComponent1::tick");
        entity.test_component2.as_ref().unwrap().borrow_mut().tell("HELLO from TestComponent1");
    }
}

#[derive(Default)]
struct TestComponent2 {
    spy: TickSpyRef
}

impl TestComponent2 {
    fn new_rc(spy: TickSpyRef) -> ComponentRef<Self> {
        Rc::new(RefCell::new(Self { spy }))
    }

    fn tell(&mut self, msg: &str) {
        self.spy.borrow_mut().log(msg);
    }
}

impl Component for TestComponent2 {
    fn tick(&mut self, _: &GameEntity) {
        self.spy.borrow_mut().log("TestComponent2::tick")
    }
}

struct InputHandler {
    input: InputRef,
    spy: TickSpyRef,
}

impl InputHandler {
    fn new_rc(spy: TickSpyRef, input: InputRef) -> ComponentRef<Self> {
        Rc::new(RefCell::new(Self { spy, input: input.clone() }))
    }
}

impl Component for InputHandler {
    fn tick(&mut self, _: &GameEntity) {
        if self.input.borrow().is_key_down(21) {
            self.spy.borrow_mut().log("GOT INPUT");
        } else {
            self.spy.borrow_mut().log("GOT NO INPUT")
        }
    }
}


#[derive(Default)]
struct GameEntity {
    test_component1: Option<ComponentRef<TestComponent1>>,
    test_component2: Option<ComponentRef<TestComponent2>>,
    input_handler: Option<ComponentRef<InputHandler>>,
    components: Vec<ComponentRef<dyn Component>>,
}

macro_rules! impl_component {
    ($c:ident, $m:ident) => {
    pub fn $m(&mut self, c: ComponentRef<$c>) {
        if self.$m.is_none() {
            self.$m = Some(c.clone());
            self.components.push(c.clone() as ComponentRef<dyn Component>)
        }
    }
}
}

impl GameEntity {
    impl_component!(TestComponent1, test_component1);
    impl_component!(TestComponent2, test_component2);
    impl_component!(InputHandler, input_handler);

    pub fn tick(&mut self) {
        for c in self.components.iter() {
            c.borrow_mut().tick(&self)
        }
    }
}

type TickSpyRef = Rc<RefCell<TickSpy>>;

#[derive(Default)]
struct TickSpy {
    log: Vec<String>
}

impl TickSpy {
    pub fn new_rc() -> TickSpyRef {
        Rc::new(RefCell::new(Self { log: Vec::with_capacity(10) }))
    }

    pub fn log(&mut self, msg: &str) {
        self.log.push(msg.to_string())
    }

    pub fn get_log(&self) -> Vec<&str> {
        self.log.iter().map(|s| s.as_str()).collect()
    }
}

#[test]
fn nothing() {
    let spy = TickSpy::new_rc();
    let mut entity = GameEntity::default();
    entity.test_component1(TestComponent1::new_rc(spy.clone()));
    entity.test_component2(TestComponent2::new_rc(spy.clone()));
    entity.test_component2(TestComponent2::new_rc(spy.clone()));

    entity.tick();

    assert_eq!(spy.borrow().get_log(), vec!["TestComponent1::tick", "HELLO from TestComponent1", "TestComponent2::tick"]);
}

#[test]
fn input_component() {
    let spy = TickSpy::new_rc();
    let mut entity = GameEntity::default();
    let input = InputStub::new_rc();
    entity.test_component2(TestComponent2::new_rc(spy.clone()));
    entity.input_handler(InputHandler::new_rc(spy.clone(), input.clone()));

    entity.tick();
    assert_eq!(spy.borrow().get_log(), vec!["TestComponent2::tick", "GOT NO INPUT"]);

    input.borrow_mut().key_is_down(21);
    entity.tick();

    assert_eq!(spy.borrow().get_log(), vec![
        "TestComponent2::tick",
        "GOT NO INPUT",
        "TestComponent2::tick",
        "GOT INPUT"
    ]);
}