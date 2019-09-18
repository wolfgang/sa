use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

type AnyRef = Box<dyn Any>;

struct AnyMap {
    values: HashMap<u32, AnyRef>
}

impl AnyMap {
    fn new() -> Self {
        Self { values: HashMap::with_capacity(100) }
    }

    fn add<T>(&mut self, key: u32, value: T) where T: Any {
        self.values.insert(key, Box::from(value));
    }

    fn get_ref<T>(&self, key: u32) -> &T where T: Any {
        let v = self.values.get(&key).unwrap();
        (*v).downcast_ref::<T>().unwrap()
    }
}

#[test]
fn any_in_rc() {
    let v: Rc<dyn Any> = Rc::new(String::from("Hello"));

    let s = v.downcast::<String>().unwrap();
    assert_eq!(s.as_str(), "Hello")
}

#[test]
fn any_in_refcell() {
    let v: Rc<RefCell<dyn Any>> = Rc::new(RefCell::new(String::from("Hello")));

    let mut x = v.borrow_mut();
    let s = x.downcast_mut::<String>().unwrap();
    s.push('x');

    assert_eq!(s.as_str(), "Hellox")
}

#[test]
fn store_any_types() {
    let mut map = AnyMap::new();

    map.add(1, String::from("hello"));
    map.add(2, 1234 as u32);

    assert_eq!(map.get_ref::<String>(1).as_str(), "hello");
    assert_eq!(map.get_ref::<u32>(2), &1234)
}