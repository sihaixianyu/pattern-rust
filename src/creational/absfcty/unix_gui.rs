use super::gui::{Button, Checkbox, GuiFactory, GuiFactoryDynamic};

pub struct UnixFactory;

impl GuiFactory for UnixFactory {
    type B = UnixButton;
    type C = UnixCheckbox;

    fn create_button(&self) -> Self::B {
        UnixButton
    }

    fn create_checkbox(&self) -> Self::C {
        UnixCheckbox
    }
}

pub struct UnixFactoryDynamic;

impl GuiFactoryDynamic for UnixFactoryDynamic {
    fn create_button(&self) -> Box<dyn Button> {
        Box::new(UnixButton)
    }

    fn create_checkbox(&self) -> Box<dyn Checkbox> {
        Box::new(UnixCheckbox)
    }
}

pub struct UnixButton;

impl Button for UnixButton {
    fn press(&self) {
        println!("pressing unix button")
    }
}

pub struct UnixCheckbox;

impl Checkbox for UnixCheckbox {
    fn switch(&self) {
        println!("switching unix checkbox")
    }
}
