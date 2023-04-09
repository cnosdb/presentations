use std::rc::Rc;
use crate::pattern::command::light::Light;
use crate::pattern::command::remote::Remote;

mod light_on_command;
mod command;
mod light;
mod remote;
mod light_off_command;

#[test]
fn test_command() {
    let light = Rc::new(Light {});
    let remote = Remote::new(light);
    remote.light_on_button();
    remote.light_off_button();
}