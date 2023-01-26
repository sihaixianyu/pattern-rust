use super::gui::{Button, Dialog};

pub struct UnixButton;

impl Button for UnixButton {
    fn render(&self) {
        println!("Drawing a Unix button");
        self.on_click();
    }

    fn on_click(&self) {
        println!("Click! Hello, Unix!");
    }
}

pub struct UnixDialog;

impl Dialog for UnixDialog {
    /// Creates a Windows button.
    fn create_button(&self) -> Box<dyn Button> {
        Box::new(UnixButton)
    }
}
