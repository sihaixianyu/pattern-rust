#[derive(Default)]
pub struct Patient {
    pub name: String,
    pub registration_done: bool,
    pub checkup_done: bool,
    pub medicine_done: bool,
    pub payment_done: bool,
}

impl Patient {
    pub fn new(name: String) -> Self {
        Self {
            name,
            registration_done: false,
            checkup_done: false,
            medicine_done: false,
            payment_done: false,
        }
    }
}
