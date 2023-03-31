pub struct Notification;

impl Notification {
    pub fn send_credit_notification(&self) {
        println!("Sending wallet credit notification");
    }

    pub fn send_debit_notification(&self) {
        println!("Sending wallet debit notification");
    }
}
