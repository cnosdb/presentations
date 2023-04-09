use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;
use crate::pattern::command::command::Command;
use crate::pattern::command::light::Light;
use crate::pattern::command::light_off_command::LightOffCommand;
use crate::pattern::command::light_on_command::LightOnCommand;

pub struct Remote {
    commands: HashMap<String, Rc<dyn Command>>,
}

impl Remote {
    pub fn new(light: Rc<Light>) -> Self {
        let mut commands: HashMap<String, Rc<dyn Command>> = HashMap::new();
        commands.insert("light on".to_string(),
                        Rc::new(LightOnCommand::new(light.clone())));
        commands.insert("light off".to_string(),
                        Rc::new(LightOffCommand::new(light.clone())));

        Self {
            commands
        }

    }

    pub fn light_on_button(&self) {
        self.commands.get("light on").map(|c| c.execute());
    }

    pub fn light_off_button(&self) {
        self.commands.get("light off").map(|c| c.execute());
    }
}
