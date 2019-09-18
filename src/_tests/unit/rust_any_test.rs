use std::any::{Any, TypeId};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

type AnyRef = Box<dyn Any>;

struct AnyMap {
    values: HashMap<TypeId, HashMap<u32, AnyRef>>,
}

impl AnyMap {
    fn new() -> Self {
        Self {
            values: HashMap::default(),
        }
    }

    fn insert<T>(&mut self, key: u32, value: T) where T: Any {
        let type_id = TypeId::of::<T>();
        let map_for_type = self.values.entry(type_id).or_insert(HashMap::with_capacity(10));
        map_for_type.insert(key, Box::from(value));
    }

    fn get_ref<T>(&self, key: u32) -> &T where T: Any {
        let type_id = TypeId::of::<T>();
        let map_for_type = self.values.get(&type_id).unwrap();
        let v = map_for_type.get(&key).unwrap();
        Self::downcast_boxed(v)
    }

    fn get_mut_ref<T>(&mut self, key: u32) -> &mut T where T: Any {
        let type_id = TypeId::of::<T>();
        let map_for_type = self.values.get_mut(&type_id).unwrap();
        let v = map_for_type.get_mut(&key).unwrap();
        (*v).downcast_mut::<T>().unwrap()
    }

    fn get_all<T>(&self) -> Vec<&T> where T: Any {
        let type_id = TypeId::of::<T>();
        let map_for_type = self.values.get(&type_id).unwrap();
        map_for_type.values().map(|v| Self::downcast_boxed(v)).collect()
    }

    fn downcast_boxed<T>(boxed: &AnyRef) -> &T where T: Any {
        (*boxed).downcast_ref::<T>().unwrap()
    }
}


type ComponentRef = Box<dyn Component>;

struct ComponentMap {
    values: HashMap<TypeId, HashMap<u32, ComponentRef>>,
}

impl ComponentMap {
    fn new() -> Self {
        Self {
            values: HashMap::default(),
        }
    }

    fn insert<T>(&mut self, key: u32, value: T) where T: Component + 'static {
        let type_id = TypeId::of::<T>();
        let map_for_type = self.values.entry(type_id).or_insert(HashMap::with_capacity(10));
        map_for_type.insert(key, Box::from(value));
    }

    fn get_ref<T>(&self, key: u32) -> &T where T: Component + 'static {
        let type_id = TypeId::of::<T>();
        let map_for_type = self.values.get(&type_id).unwrap();
        let v = map_for_type.get(&key).unwrap();
        Self::downcast_boxed(v)
    }

    fn get_mut_ref<T>(&mut self, key: u32) -> &mut T where T: Component + 'static {
        let type_id = TypeId::of::<T>();
        let map_for_type = self.values.get_mut(&type_id).unwrap();
        let v = map_for_type.get_mut(&key).unwrap();
        (*v).as_any_mut().downcast_mut::<T>().unwrap()
    }

    fn get_all<T>(&self) -> Vec<&T> where T: Component + 'static {
        let type_id = TypeId::of::<T>();
        let map_for_type = self.values.get(&type_id).unwrap();
        map_for_type.values().map(|v| Self::downcast_boxed(v)).collect()
    }

    fn get_all_components(&self) -> Vec<&Box<dyn Component>> {
        let mut result = Vec::new();

        for v in self.values.values() {
            for b in v.values() {
                result.push(b);
            }
        }

        result
    }

    fn downcast_boxed<T>(boxed: &ComponentRef) -> &T where T: Component + 'static {
        (*boxed).as_any().downcast_ref::<T>().unwrap()
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

trait SomeTrait {
    fn get_x(&self) -> i32;
}

#[derive(PartialEq, Debug)]
struct SomeStruct {
    x: i32
}

impl SomeTrait for SomeStruct {
    fn get_x(&self) -> i32 {
        self.x
    }
}


#[test]
fn store_any_types_by_key() {
    let mut map = AnyMap::new();

    map.insert(1, String::from("hello"));
    map.insert(2, String::from("there"));
    map.insert(1, 1234 as u32);
    map.insert(17, 5678 as u32);
    map.insert(1, SomeStruct { x: 1234 });

    assert_eq!(map.get_ref::<String>(1).as_str(), "hello");
    assert_eq!(map.get_ref::<String>(2).as_str(), "there");

    assert_eq!(map.get_ref::<u32>(1), &1234);
    assert_eq!(map.get_ref::<u32>(17), &5678);

    assert_eq!(map.get_ref::<SomeStruct>(1), &SomeStruct { x: 1234 });

    let s = map.get_mut_ref::<String>(2);
    s.push('x');
    assert_eq!(map.get_ref::<String>(2).as_str(), "therex");


    let s = map.get_mut_ref::<SomeStruct>(1);
    s.x = 5678;
    assert_eq!(map.get_ref::<SomeStruct>(1), &SomeStruct { x: 5678 })
}

#[test]
fn get_struct_as_trait_object() {
    let mut map = AnyMap::new();

    map.insert(1, SomeStruct { x: 1234 });
    assert_eq!(map.get_ref::<SomeStruct>(1), &SomeStruct { x: 1234 });

    let trait_object = map.get_ref::<SomeStruct>(1) as &dyn SomeTrait;
    assert_eq!(trait_object.get_x(), 1234)
}

#[test]
fn get_all_values_for_type() {
    let mut map = AnyMap::new();
    map.insert(1, String::from("one"));
    map.insert(2, String::from("two"));

    let values = map.get_all::<String>();

    assert_eq!(values.len(), 2);
    assert!(values.contains(&&String::from("one")));
    assert!(values.contains(&&String::from("two")));
}

trait AsAny: Any {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}


trait Component: AsAny {}

#[derive(PartialEq, Debug)]
struct SomeComponent {
    x: i32
}

impl Component for SomeComponent {}

#[derive(PartialEq, Debug)]
struct SomeOtherComponent {
    y: u32
}

impl Component for SomeOtherComponent {}


impl<T: Component + 'static> AsAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

#[test]
fn component_map() {
    let mut map = ComponentMap::new();

    map.insert(1, SomeComponent { x: 1234 });
    map.insert(1, SomeOtherComponent { y: 1234 });
    map.insert(2, SomeComponent { x: 5678 });
    assert_eq!(map.get_ref::<SomeComponent>(1), &SomeComponent { x: 1234 });
    assert_eq!(map.get_ref::<SomeOtherComponent>(1), &SomeOtherComponent { y: 1234 });
    assert_eq!(map.get_ref::<SomeComponent>(2), &SomeComponent { x: 5678 });

    let all = map.get_all_components();
    assert_eq!(all.len(), 3);
    let c = all[0];
}