pub mod editor;
pub mod observer;

pub fn save_listener(file_path: String) {
    let email = "anynomous@gmail.com".to_string();
    println!("Email to {}: Save file {}", email, file_path);
}

#[cfg(test)]
mod tests {
    use super::{editor::Editor, observer::Event, save_listener};

    #[test]
    fn test_observer() {
        let mut editor = Editor::default();
        editor.publisher.subscribe(Event::Load, |file_path| {
            println!("Loading file from {}", file_path);
        });

        editor.publisher.subscribe(Event::Save, save_listener);

        editor.load("test_1.txt".into());
        editor.load("test_2.txt".into());
        editor.save();

        editor.publisher.unsubscribe(Event::Save, save_listener);

        editor.load("test_3.txt".into());
        editor.save();
    }
}
