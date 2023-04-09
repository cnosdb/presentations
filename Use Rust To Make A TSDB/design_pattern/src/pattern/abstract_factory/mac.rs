use crate::pattern::abstract_factory::ui::{Button, CheckBox, GuiFactory, GuiFactoryDyn};

pub struct MacButton;

impl Button for MacButton {
    fn press(&self) {
        println!("mac button");
    }
}

pub struct MacCheckBox;

impl CheckBox for MacCheckBox {
    fn switch(&self) {
        println!("mac switch")
    }
}

pub struct MacFactory;

impl GuiFactory for MacFactory {
    type B = MacButton;
    type C = MacCheckBox;

    fn create_button(&self) -> Self::B {
        MacButton
    }

    fn create_checkbox(&self) -> Self::C {
        MacCheckBox
    }
}

impl GuiFactoryDyn for MacFactory {
    fn create_button(&self) -> Box<dyn Button> {
        Box::new(MacButton)
    }

    fn create_checkbox(&self) -> Box<dyn CheckBox> {
        Box::new(MacCheckBox)
    }
}
