use std::sync::Arc;
use crate::pattern::observer::editor::Editor;
use crate::pattern::observer::event::Event;
use crate::pattern::observer::subscriber::{LoadFileSubscriber, SaveFileSubscriber, Subscriber};

mod event;
mod publisher;
mod subscriber;
mod editor;

#[test]
fn test_observer() {
    let mut editor = Editor::default();
    editor.publisher().subscribe(Event::Load, Arc::new(LoadFileSubscriber {}));
    editor.publisher().subscribe(Event::Save, Arc::new(SaveFileSubscriber {}));
    editor.load("hello_world".to_string());
    editor.save();

}