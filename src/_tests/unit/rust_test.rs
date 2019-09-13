use std::cell::RefCell;
use std::rc::Rc;

struct Test1 {
    x: u32
}

impl Test1 {
    pub fn inc(&mut self) {
        self.x += 1;
        self.inc_2();
    }

    fn inc_2(&mut self) {
        self.x += 2
    }
}

#[test]
fn call_mut_function_from_mut_function() {
    let mut obj = Test1 { x: 11 };
    obj.inc();
    assert_eq!(obj.x, 14);
}

trait TestTrait {
    fn add_this(&self, v: u32) -> u32;
}


struct TestTraitImpl {
    base: u32
}

impl TestTrait for TestTraitImpl {
    fn add_this(&self, v: u32) -> u32 {
        self.base + v
    }
}

type TraitRef = Rc<RefCell<dyn TestTrait>>;

#[test]
fn vec_of_trait_objects() {
    let mut v: Vec<TraitRef> = Vec::new();

    let obj = Rc::new(RefCell::new(TestTraitImpl { base: 100 }));
    v.push(obj.clone());

    assert_eq!(v[0].borrow().add_this(20), 120);
}