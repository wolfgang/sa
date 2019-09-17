use std::cell::RefCell;
use std::rc::Rc;

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



#[derive(Default)]
struct GameEntity {
    test_component1: Option<ComponentRef<TestComponent1>>,
    test_component2: Option<ComponentRef<TestComponent2>>,
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