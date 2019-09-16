use std::cell::RefCell;
use std::rc::Rc;

trait Event {}

trait EventHandler<T> where T: Event {
    fn handle(&mut self, event: T);
}

#[derive(Default, Clone, Copy)]
struct Event1 { int_val: i32 }

#[derive(Default, Clone, Copy)]
struct Event2 { float_val: f32 }

impl Event for Event1 {}

impl Event for Event2 {}


#[derive(Default)]
struct TestEventHandler {
    int_val_received: i32,
    float_val_received: f32,
}

impl EventHandler<Event1> for TestEventHandler {
    fn handle(&mut self, event: Event1) {
        self.int_val_received = event.int_val;
    }
}

impl EventHandler<Event2> for TestEventHandler {
    fn handle(&mut self, event: Event2) {
        self.float_val_received = event.float_val;
    }
}

type EventHandlerRef<T> = Rc<RefCell<dyn EventHandler<T>>>;

trait EventDispatcher<T> where T: Event {
    fn on(&mut self, evt: T, handler: EventHandlerRef<T>);
}

#[derive(Default)]
struct TestEventDispatcher {
    handlers_event1: Vec<EventHandlerRef<Event1>>,
    handlers_event2: Vec<EventHandlerRef<Event2>>,
}

impl TestEventDispatcher {
    fn on_event1(&mut self, handler: EventHandlerRef<Event1>) {
        self.handlers_event1.push(handler.clone());
    }
    fn on_event2(&mut self, handler: EventHandlerRef<Event2>) {
        self.handlers_event2.push(handler.clone());
    }

    fn dispatch_event1(&self, evt: Event1) {
        for handler in self.handlers_event1.iter() {
            handler.borrow_mut().handle(evt);
        }
    }

    fn dispatch_event2(&self, evt: Event2) {
        for handler in self.handlers_event2.iter() {
            handler.borrow_mut().handle(evt);
        }
    }

}


#[test]
fn nothing() {
    let mut handler = TestEventHandler::default();
    handler.handle(Event1 { int_val: 123 });
    assert_eq!(handler.int_val_received, 123);
    handler.handle(Event2 { float_val: 456.789 });
    assert_eq!(handler.float_val_received, 456.789);
}

#[test]
fn dispatching_of_events() {
    let mut dispatcher = TestEventDispatcher::default();
    let handler = Rc::new(RefCell::new(TestEventHandler::default()));
    dispatcher.on_event1(handler.clone());
    dispatcher.on_event2(handler.clone());

    assert_eq!(1, dispatcher.handlers_event1.len());
    assert_eq!(1, dispatcher.handlers_event2.len());

    dispatcher.dispatch_event1(Event1 { int_val: 17 });
    dispatcher.dispatch_event2(Event2 { float_val: 19.31 });
    assert_eq!(handler.borrow().int_val_received, 17);
    assert_eq!(handler.borrow().float_val_received, 19.31);
}