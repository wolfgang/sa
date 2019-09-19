use std::any::{Any, TypeId};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

macro_rules! gen_any_map {
    ($name:ident, $base_type:ident) => {
        struct $name {
            values: HashMap<TypeId, HashMap<u32, Box<dyn $base_type>>>,
        }

        impl $name {
            fn new() -> Self {
                Self {
                    values: HashMap::default(),
                }
            }

            fn insert<T>(&mut self, key: u32, value: T) where T: $base_type + AsAny + 'static {
                let type_id = TypeId::of::<T>();
                let map_for_type = self.values.entry(type_id).or_insert(HashMap::with_capacity(10));
                map_for_type.insert(key, Box::from(value));
            }

            fn get_ref<T>(&self, key: u32) -> &T where T: $base_type + AsAny + 'static {
                let type_id = TypeId::of::<T>();
                let map_for_type = self.values.get(&type_id).unwrap();
                let v = map_for_type.get(&key).unwrap();
                Self::downcast_boxed(v)
            }

            fn get_mut_ref<T>(&mut self, key: u32) -> &mut T where T: $base_type + AsAny + 'static {
                let type_id = TypeId::of::<T>();
                let map_for_type = self.values.get_mut(&type_id).unwrap();
                let v = map_for_type.get_mut(&key).unwrap();
                (*v).as_any_mut().downcast_mut::<T>().unwrap()
            }

            fn get_all_of_type<T>(&self) -> Vec<&T> where T: $base_type + AsAny + 'static {
                let type_id = TypeId::of::<T>();
                let map_for_type = self.values.get(&type_id).unwrap();
                map_for_type.values().map(|v| Self::downcast_boxed(v)).collect()
            }

            fn get_all_as_base(&self) -> Vec<&Box<dyn $base_type>> {
                let mut result = Vec::new();

                for v in self.values.values() {
                    for b in v.values() {
                        result.push(b);
                    }
                }

                result
            }

            fn downcast_boxed<T>(boxed: &Box<dyn $base_type>) -> &T where T: $base_type + AsAny + 'static {
                (*boxed).as_any().downcast_ref::<T>().unwrap()
            }
        }
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

trait AsAny: Any {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}


trait Component: AsAny {
    fn get_value(&self) -> i32;
}

#[derive(PartialEq, Debug)]
struct SomeComponent {
    x: i32
}

impl Component for SomeComponent {
    fn get_value(&self) -> i32 {
        self.x
    }
}

#[derive(PartialEq, Debug)]
struct SomeOtherComponent {
    y: u32
}

impl Component for SomeOtherComponent {
    fn get_value(&self) -> i32 {
        self.y as i32
    }
}


impl<T: Component + 'static> AsAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

gen_any_map!(ComponentMap, Component);

#[test]
fn component_map() {
    let mut map = ComponentMap::new();

    map.insert(1, SomeComponent { x: 1234 });
    map.insert(1, SomeOtherComponent { y: 1234 });
    map.insert(2, SomeComponent { x: 5678 });
    assert_eq!(map.get_ref::<SomeComponent>(1), &SomeComponent { x: 1234 });
    assert_eq!(map.get_ref::<SomeOtherComponent>(1), &SomeOtherComponent { y: 1234 });
    assert_eq!(map.get_ref::<SomeComponent>(2), &SomeComponent { x: 5678 });

    let all = map.get_all_as_base();
    assert_eq!(all.len(), 3);
    assert!(all[0].get_value() > 0);
    assert!(all[1].get_value() > 0);
    assert!(all[2].get_value() > 0);

    let all_somes = map.get_all_of_type::<SomeComponent>();
    assert_eq!(all_somes.len(), 2);

    assert!(all_somes.contains(&&SomeComponent { x: 1234 }));
    assert!(all_somes.contains(&&SomeComponent { x: 5678 }));

    let mutable_ref = map.get_mut_ref::<SomeOtherComponent>(1);
    mutable_ref.y = 5678;
    assert_eq!(map.get_ref::<SomeOtherComponent>(1), &SomeOtherComponent { y: 5678 });
}