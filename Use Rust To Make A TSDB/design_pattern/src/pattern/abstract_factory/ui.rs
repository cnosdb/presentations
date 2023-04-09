pub trait Button {
    fn press(&self);
}

pub trait CheckBox {
    fn switch(&self);
}

pub trait GuiFactory {
    type B: Button;
    type C: CheckBox;

    fn create_button(&self) -> Self::B;
    fn create_checkbox(&self) -> Self::C;
}

pub trait GuiFactoryDyn {
    fn create_button(&self) -> Box<dyn Button>;
    fn create_checkbox(&self) -> Box<dyn CheckBox>;
}
