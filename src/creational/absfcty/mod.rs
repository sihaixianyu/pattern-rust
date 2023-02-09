pub mod client;
pub mod gui;
pub mod unix_gui;
pub mod windows_gui;

#[cfg(test)]
mod tests {
    use super::{client, unix_gui::{UnixFactory, UnixFactoryDynamic}, windows_gui::{WindowsFactory, WindowsFactoryDynamic}, gui::GuiFactoryDynamic};


    #[test]
    fn test_static_dispatch() {
        if cfg!(unix) {
            client::render(UnixFactory);
        } else if cfg!(windows) {
            client::render(WindowsFactory);
        } else {
            panic!("unsupported platform");
        }
    }

    #[test]
    fn test_dynamic_dispatch() {
        let fcty: &dyn GuiFactoryDynamic = if cfg!(unix) {
            &UnixFactoryDynamic
        } else if cfg!(windows) {
            &WindowsFactoryDynamic
        } else {
            panic!("unsupported platform");
        };

        let btn = fcty.create_button();
        let chkbox = fcty.create_checkbox();

        btn.press();
        chkbox.switch();
    }
}
