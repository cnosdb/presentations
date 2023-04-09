use crate::pattern::observer::event::Event;
use crate::pattern::observer::publisher::Publisher;

#[derive(Default)]
pub struct Editor {
    file_path: String,
    publisher: Publisher,
}

impl Editor {
    pub fn publisher(&mut self) -> &mut Publisher {
        &mut self.publisher
    }
    pub fn load(&mut self, path: String) {
        self.file_path = path.clone();
        self.publisher.notify(Event::Load, path.clone());
    }

    pub fn save(&self) {
        self.publisher.notify(Event::Save, self.file_path.clone())
    }
}