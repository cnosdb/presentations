// pub mod mac;
// pub mod ui;
// pub mod windows;
//
// #[cfg(test)]
// mod test {
//     use crate::pattern::abstract_factory::mac::MacFactory;
//     use crate::pattern::abstract_factory::ui::{Button, CheckBox, GuiFactory, GuiFactoryDyn};
//     use crate::pattern::abstract_factory::windows::WindowsFactory;
//
//     pub fn test(factory: impl GuiFactory) {
//         let button1 = factory.create_button();
//         let button2 = factory.create_button();
//         let checkbox1 = factory.create_checkbox();
//         let checkbox2 = factory.create_checkbox();
//
//         button1.press();
//         button2.press();
//         checkbox1.switch();
//         checkbox2.switch();
//     }
//
//     pub fn test_dyn(factory: &dyn GuiFactoryDyn) {
//         let button1 = factory.create_button();
//         let button2 = factory.create_button();
//         let checkbox1 = factory.create_checkbox();
//         let checkbox2 = factory.create_checkbox();
//
//         button1.press();
//         button2.press();
//         checkbox1.switch();
//         checkbox2.switch();
//     }
//
//     #[test]
//     fn test_ab_fac() {
//         let windows = true;
//
//         if windows {
//             test(WindowsFactory)
//         } else {
//             test(MacFactory)
//         }
//     }
//
//     #[test]
//     fn test_dyn_ab_fac() {
//         let windows = true;
//         let factory: &dyn GuiFactoryDyn = if windows {
//             &WindowsFactory
//         } else {
//             &MacFactory
//         };
//         let button = factory.create_button();
//
//         test_dyn(factory)
//     }
// }
