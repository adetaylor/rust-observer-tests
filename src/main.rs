trait Observer {
    fn event_occurred(&self);
}

struct ObserverRegistry {
    observers: Vec<Box<dyn Observer>>
}

impl ObserverRegistry {
    fn new() -> Self {
        ObserverRegistry {
            observers: Vec::new()
        }
    }

    fn notify(&self) {
        for a in self.observers {
            a.event_occurred();
        }
    }

    fn register(&mut self, observer: &mut Observer) {
        self.observers.push(observer);
    }
}

struct EventGenerator {
    observers: ObserverRegistry
}

impl EventGenerator {
    fn new() -> Self {
        EventGenerator {
            observers: ObserverRegistry::new()
        }
    }
}

impl EventGenerator {
    fn do_something(&self) {
        println!("About to notify observers");
        self.observers.notify();
        println!("Finished notifying observers");
    }

    fn register(&mut self, observer: &mut Observer) {
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
    counter: usize
}

impl Observer for EventConsumerA {
    fn event_occurred(&self) {
        self.handle_event_which_occurred();
    }
}

impl EventConsumerB {
    fn new() -> Self {
        EventConsumerB{
            counter: 0
        }
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
    fn event_occurred(&self) {
        self.handle_event_which_occurred();
    }
}

fn main() {
    let gen = EventGenerator::new();
    let consumer_a = EventConsumerA{};
    let mut consumer_b = EventConsumerB::new();

    gen.register(&consumer_a);
    gen.register(&mut consumer_b);

    gen.do_something();
    gen.do_something();

    consumer_b.report();

    println!("Hello, world!");
}
