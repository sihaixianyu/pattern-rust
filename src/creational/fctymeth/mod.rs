use self::gui::Dialog;

pub mod gui;
pub mod html_gui;
pub mod unix_gui;
pub mod windows_gui;

use html_gui::HtmlDialog;
use unix_gui::UnixDialog;
use windows_gui::WindowsDialog;

pub fn get_dialog() -> &'static dyn Dialog {
    if cfg!(unix) {
        println!("-- Unix detected, creating Unix GUI --");
        &UnixDialog
    } else if cfg!(windows) {
        println!("-- Windows detected, creating Windows GUI --");
        &WindowsDialog
    } else {
        println!("-- No OS detected, creating the HTML GUI --");
        &HtmlDialog
    }
}

#[cfg(test)]
mod tests {
    use super::get_dialog;

    #[test]
    fn test_get_dialog() {
        let dialog = get_dialog();
        dialog.render();
        dialog.refresh();
    }
}
