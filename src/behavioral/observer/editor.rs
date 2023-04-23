use super::observer::{Event, Publisher};

#[derive(Default)]
pub struct Editor {
    pub publisher: Publisher,
    file_path: String,
}

impl Editor {
    pub fn load(&mut self, file_path: String) {
        self.file_path = file_path.clone();
        self.publisher.notify(Event::Load, file_path);
    }

    pub fn save(&mut self) {
        self.publisher.notify(Event::Save, self.file_path.clone());
    }
}
