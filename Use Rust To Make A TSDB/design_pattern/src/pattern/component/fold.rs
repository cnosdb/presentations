use std::ops::AddAssign;
use crate::pattern::component::component::Component;

pub struct Folder {
    name: &'static str,
    children: Vec<Box<dyn Component>>
}

impl Folder {
    pub fn new(name: &'static str) -> Self {
        Self {
            name,
            children: vec![]
        }
    }

    pub fn add_component(&mut self, component: impl Component + 'static) {
        let component = Box::new(component);
        self.children.push(component)
    }
}

impl Component for Folder {
    fn search(&self, keyword: &str) {
        println!("Searching for key word {keyword} in folder {}", self.name);
        self.children.iter().for_each(|c| c.search(keyword))
    }
    fn name(&self) -> &'static str {
        self.name
    }
}

pub struct FolderIterator {
    folder: Folder,
    flag: bool,
    index: usize,
}
