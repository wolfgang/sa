use std::any::Any;
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

#[test]
fn any_in_rc() {
    let v: Rc<dyn Any> = Rc::new(String::from("Hello"));

    let s = v.downcast::<String>().unwrap();
    assert_eq!(s.as_str(), "Hello")
}

#[test]
fn any_in_refcell() {
    let v: Rc<RefCell<dyn Any>> = Rc::new(RefCell::new(String::from("Hello")));

    let x = v.borrow();
    let s = x.deref().downcast_ref::<String>().unwrap();

    assert_eq!(s.as_str(), "Hello")
}