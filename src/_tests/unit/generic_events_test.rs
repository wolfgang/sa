use std::cell::RefCell;
use std::rc::Rc;

const EVENT1: u32 = 1;
const EVENT2: u32 = 2;

trait Event {
    fn id(&self) -> u32;
}

trait EventHandler<T> where T: Event {
    fn handle(&mut self, event: T);
}

struct Event1 { int_val: i32 }

struct Event2 { float_val: f32 }

impl Event for Event1 {
    fn id(&self) -> u32 {
        EVENT1
    }
}

impl Event for Event2 {
    fn id(&self) -> u32 {
        EVENT2
    }
}


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
    fn on(&mut self, handler: EventHandlerRef<T>);
}

#[derive(Default)]
struct TestEventDispatcher {
    handlers_event1: Vec<EventHandlerRef<Event1>>,
    handlers_event2: Vec<EventHandlerRef<Event2>>,
}

impl EventDispatcher<Event1> for TestEventDispatcher {
    fn on(&mut self, handler: EventHandlerRef<Event1>) {
        self.handlers_event1.push(handler.clone());
    }
}

impl EventDispatcher<Event2> for TestEventDispatcher {
    fn on(&mut self, handler: EventHandlerRef<Event2>) {
        self.handlers_event2.push(handler.clone());
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
    dispatcher.on(handler.clone() as Rc<RefCell<dyn EventHandler<Event1>>>);
    dispatcher.on(handler.clone() as Rc<RefCell<dyn EventHandler<Event2>>>);

    assert_eq!(1, dispatcher.handlers_event1.len());
    assert_eq!(1, dispatcher.handlers_event2.len());
}