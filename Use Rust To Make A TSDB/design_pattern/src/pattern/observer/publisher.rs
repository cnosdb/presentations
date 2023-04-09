use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Sub;
use std::sync::Arc;
use crate::pattern::observer::event::Event;
use crate::pattern::observer::subscriber::Subscriber;

#[derive(Default)]
pub struct Publisher {
    events: HashMap<Event, Vec<Arc<dyn Subscriber>>>
}

impl Publisher {
    pub fn subscribe(&mut self, event_type: Event, listener: Arc<dyn Subscriber>) {
        let subscribers = self.events.entry(event_type).or_default();
        subscribers.push(listener);
    }

    pub fn unsubscribe(&mut self, event_type: Event, listener: Arc<dyn Subscriber>) {
        self
            .events
            .get_mut(&event_type)
            .map(|s|
                s.retain(|x| x.id() != listener.id()));
    }

    pub fn notify(&self, event_type: Event, file_path: String) {
        self.events.get(&event_type).map(|s| s.iter().for_each(
            |s| s.update(event_type, file_path.clone())));

    }
}