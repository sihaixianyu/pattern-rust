use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq, Clone)]
pub enum Event {
    Load,
    Save,
}

pub type Subscriber = fn(file_path: String);

#[derive(Default)]
pub struct Publisher {
    events: HashMap<Event, Vec<Subscriber>>,
}

impl Publisher {
    pub fn subscribe(&mut self, event: Event, subscriber: Subscriber) {
        self.events.entry(event.clone()).or_default();
        self.events.get_mut(&event).unwrap().push(subscriber);
    }

    pub fn unsubscribe(&mut self, event: Event, listener: Subscriber) {
        if let Some(subscribers)= self.events.get_mut(&event) {
            subscribers.retain(|&x| x != listener);
        }
    }

    pub fn notify(&mut self, event: Event, file_path: String) {
        if let Some(subscribers)= self.events.get_mut(&event) {
            for listener in subscribers {
                listener(file_path.clone());
            }
        }
    }
}
