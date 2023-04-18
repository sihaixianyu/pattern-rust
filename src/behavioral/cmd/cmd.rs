use super::Receiver;

pub trait Command {
    fn execute(&self);
}

pub struct SimpleCommand {
    pay_load: String,
}

impl SimpleCommand {
    pub fn new(pay_load: String) -> Self {
        Self { pay_load }
    }
}

impl Command for SimpleCommand {
    fn execute(&self) {
        println!(
            "SimpleCommand: See, I can do simple things like printing {}",
            self.pay_load
        );
    }
}

pub struct ComplexCommand {
    receiver: Box<Receiver>,

    a: String,
    b: String,
}

impl ComplexCommand {
    pub fn new(receiver: Box<Receiver>, a: String, b: String) -> ComplexCommand {
        Self { receiver, a, b }
    }
}

impl<'a> Command for ComplexCommand {
    fn execute(&self) {
        println!("ComplexCommand: Complex stuff should be done by a receiver object.");
        self.receiver.do_something(&self.a);
        self.receiver.do_something_else(&self.b);
    }
}
