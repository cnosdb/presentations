use crate::pattern::component::component::Component;

pub struct File {
    name: &'static str,
}

impl File {
    pub fn new(name: &'static str) -> Self {
        Self {
            name
        }
    }
}

impl Component for File {
    fn search(&self, keyword: &str) {
        println!("Searching for keyword {} in file {}", keyword, self.name);
    }
    fn name(&self) -> &'static str {
        self.name
    }
}