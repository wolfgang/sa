use std::any::{Any, TypeId};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

type AnyRef = Box<dyn Any>;

struct AnyMap {
    values: HashMap<TypeId, AnyRef>
}

impl AnyMap {
    fn new() -> Self {
        Self {
            values: HashMap::default()
        }
    }

    fn add<T>(&mut self, value: T) where T: Any {
        self.values.insert(TypeId::of::<T>(), Box::from(value));
    }

    fn get_ref<T>(&self) -> &T where T: Any {
        let v = self.values.get(&TypeId::of::<T>()).unwrap();
        (*v).downcast_ref::<T>().unwrap()
    }

    fn get_mut_ref<T>(&mut self) -> &mut T where T: Any {
        let v = self.values.get_mut(&TypeId::of::<T>()).unwrap();
        (*v).downcast_mut::<T>().unwrap()
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


struct SomeStruct {
    x: i32
}


#[test]
fn store_any_types() {
    let mut map = AnyMap::new();

    map.add(String::from("hello"));
    map.add(1234 as u32);

    assert_eq!(map.get_ref::<String>().as_str(), "hello");
    assert_eq!(map.get_ref::<u32>(), &1234);

    let mut_str = map.get_mut_ref::<String>();
    mut_str.push('x');
    assert_eq!(map.get_ref::<String>().as_str(), "hellox");

    map.add(SomeStruct { x: 1234 });

    let s = map.get_ref::<SomeStruct>();
    assert_eq!(s.x, 1234)



}