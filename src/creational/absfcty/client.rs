use super::gui::{Button, Checkbox, GuiFactory, GuiFactoryDynamic};

pub fn render(factory: impl GuiFactory) {
    let btn = factory.create_button();
    let chkbox = factory.create_checkbox();

    btn.press();
    chkbox.switch();
}

pub fn render_dynamic(factory: &dyn GuiFactoryDynamic) {
    let button1 = factory.create_button();
    let button2 = factory.create_button();
    let checkbox1 = factory.create_checkbox();
    let checkbox2 = factory.create_checkbox();

    button1.press();
    button2.press();
    checkbox1.switch();
    checkbox2.switch();
}