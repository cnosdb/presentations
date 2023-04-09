use std::rc::Rc;
use crate::pattern::command::command::Command;
use crate::pattern::command::light::Light;

pub struct LightOffCommand {
    light: Rc<Light>
}
impl LightOffCommand {
    pub fn new(light: Rc<Light>) -> Self {
        Self {
            light
        }
    }
}

impl Command for LightOffCommand {
    fn execute(&self) {
        self.light.off()
    }
}
