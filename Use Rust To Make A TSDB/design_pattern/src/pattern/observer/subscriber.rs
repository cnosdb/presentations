use crate::pattern::observer::event::Event;

pub trait Subscriber {
    fn id(&self) -> String;
    fn update(&self, event: Event, file_path: String);
}

pub struct LoadFileSubscriber {}

impl Subscriber for LoadFileSubscriber {
    fn id(&self) -> String {
        "load_file_subscriber".to_string()
    }

    fn update(&self, event: Event, file_path: String) {
        println!("load {file_path}")
    }
}

pub struct SaveFileSubscriber {}
impl Subscriber for SaveFileSubscriber {
    fn id(&self) -> String {
        "save_file_subscriber".to_string()
    }

    fn update(&self, event: Event, file_path: String) {
        println!("save {file_path}")
    }
}

