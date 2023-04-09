use crate::pattern::abstract_factory::ui::{Button, CheckBox, GuiFactory, GuiFactoryDyn};

pub struct WindowsButton;

impl Button for WindowsButton {
    fn press(&self) {
        println!("Windows button");
    }
}

pub struct WindowsCheckBox;

impl CheckBox for WindowsCheckBox {
    fn switch(&self) {
        println!("Windows switch")
    }
}

pub struct WindowsFactory;

impl GuiFactory for WindowsFactory {
    type B = WindowsButton;
    type C = WindowsCheckBox;

    fn create_button(&self) -> Self::B {
        WindowsButton
    }

    fn create_checkbox(&self) -> Self::C {
        WindowsCheckBox
    }
}

impl GuiFactoryDyn for WindowsFactory {
    fn create_button(&self) -> Box<dyn Button> {
        Box::new(WindowsButton)
    }

    fn create_checkbox(&self) -> Box<dyn CheckBox> {
        Box::new(WindowsCheckBox)
    }
}
