trait Event {}

trait EventHandler<T> where T: Event {
    fn handle(&mut self, event: T);
}

struct Event1 { int_val: i32 }

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


#[test]
fn nothing() {
    let mut handler = TestEventHandler::default();
    handler.handle(Event1 { int_val: 123 });
    assert_eq!(handler.int_val_received, 123);
    handler.handle(Event2 { float_val: 456.789 });
    assert_eq!(handler.float_val_received, 456.789);
}