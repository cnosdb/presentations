use std::rc::Rc;
use crate::pattern::command::command::Command;
use crate::pattern::command::light::Light;

pub struct LightOnCommand {
    light: Rc<Light>
}

impl LightOnCommand {
    pub fn new(light: Rc<Light>) -> Self {
        Self {
            light
        }
    }
}

impl Command for LightOnCommand {
    fn execute(&self) {
        self.light.on()
    }
}

#[test]
fn test_light_on_command() {
    let command = LightOnCommand {
        light: Rc::new(Light{})
    };
    command.execute();
}