use std::cell::RefCell;
use std::rc::{Rc, Weak};

trait Observer {
    fn event_occurred(&mut self);
}

struct ObserverRegistry {
    observers: Vec<Weak<RefCell<dyn Observer>>>,
}

impl ObserverRegistry {
    fn new() -> Self {
        ObserverRegistry {
            observers: Vec::new(),
        }
    }

    fn notify(&mut self) {
        for a in &self.observers {
            if let Some(a) = a.upgrade() {
                a.borrow_mut().event_occurred();
            }
        }
    }

    fn register(&mut self, observer: Weak<RefCell<dyn Observer>>) {
        self.observers.push(observer);
    }
}

struct EventGenerator {
    observers: ObserverRegistry,
}

impl EventGenerator {
    fn new() -> Self {
        EventGenerator {
            observers: ObserverRegistry::new(),
        }
    }
}

impl EventGenerator {
    fn do_something(&mut self) {
        println!("About to notify observers");
        self.observers.notify();
        println!("Finished notifying observers");
    }

    fn register(&mut self, observer: Weak<RefCell<dyn Observer>>) {
        self.observers.register(observer);
    }
}

struct EventConsumerA;

impl EventConsumerA {
    fn handle_event_which_occurred(&self) {
        println!("A: discovered event");
    }
}

struct EventConsumerB {
    counter: usize,
}

impl Observer for EventConsumerA {
    fn event_occurred(&mut self) {
        self.handle_event_which_occurred();
    }
}

impl EventConsumerB {
    fn new() -> Self {
        EventConsumerB { counter: 0 }
    }

    fn handle_event_which_occurred(&mut self) {
        self.counter += 1;
        println!("B: discovered event: counter is {}", self.counter);
    }

    fn report(&self) {
        println!("Final counter is {}", self.counter);
    }
}

impl Observer for EventConsumerB {
    fn event_occurred(&mut self) {
        self.handle_event_which_occurred();
    }
}

fn main() {
    let mut gen = EventGenerator::new();
    let consumer_a = Rc::new(RefCell::new(EventConsumerA {}));
    let consumer_b = Rc::new(RefCell::new(EventConsumerB::new()));

    let listener_a = Rc::downgrade(&consumer_a);
    gen.register(listener_a);
    let listener_b = Rc::downgrade(&consumer_b);
    gen.register(listener_b);

    gen.do_something();
    gen.do_something();

    consumer_b.borrow().report();

    println!("Hello, world!");
}
