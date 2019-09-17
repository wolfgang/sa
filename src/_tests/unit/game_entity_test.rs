use std::cell::RefCell;
use std::rc::Rc;

type ComponentRef<T> = Rc<RefCell<T>>;

trait Component {
    fn tick(&mut self);
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
    fn tick(&mut self) {
        self.spy.borrow_mut().log("TestComponent1::tick")
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
}

impl Component for TestComponent2 {
    fn tick(&mut self) {
        self.spy.borrow_mut().log("TestComponent2::tick")
    }
}


#[derive(Default)]
struct GameEntity {
    test_component1: Option<ComponentRef<TestComponent1>>,
    test_component2: Option<ComponentRef<TestComponent2>>,
    components: Vec<ComponentRef<dyn Component>>,
}

impl GameEntity {
    pub fn add_test_component1(&mut self, c: ComponentRef<TestComponent1>) {
        self.test_component1 = Some(c.clone());
        self.components.push(c.clone() as ComponentRef<dyn Component>)
    }

    pub fn add_test_component2(&mut self, c: ComponentRef<TestComponent2>) {
        self.test_component2 = Some(c.clone());
        self.components.push(c.clone() as ComponentRef<dyn Component>)
    }

    pub fn tick(&mut self) {
        for c in self.components.iter() {
            c.borrow_mut().tick()
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
    entity.add_test_component1(TestComponent1::new_rc(spy.clone()).clone());
    entity.add_test_component2(TestComponent2::new_rc(spy.clone()).clone());

    entity.tick();

    assert_eq!(spy.borrow().get_log(), vec!["TestComponent1::tick", "TestComponent2::tick"]);
}