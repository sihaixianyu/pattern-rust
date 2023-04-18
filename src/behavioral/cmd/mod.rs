pub mod cmd;

use self::cmd::Command;

#[derive(Default)]
pub struct Receiver;

impl Receiver {
    pub fn do_something(&self, a: &str) {
        println!("Receiver is working on: {}", a);
    }

    pub fn do_something_else(&self, b: &str) {
        println!("Receiver is also working on: {}", b);
    }
}

#[derive(Default)]
pub struct Invoker {
    on_start: Option<Box<dyn Command>>,
    on_finish: Option<Box<dyn Command>>,
}

impl Invoker {
    pub fn set_on_start(&mut self, cmd: Box<dyn Command>) {
        self.on_start = Some(cmd);
    }

    pub fn set_on_finish(&mut self, cmd: Box<dyn Command>) {
        self.on_finish = Some(cmd);
    }

    pub fn do_something_important(&self) {
        println!("Invoker: Does anybody want something done before I begin?");

        if let Some(on_start) = self.on_start.as_ref() {
            on_start.execute();
        }

        println!("Invoker: ...doing something really important...");
        println!("Invoker: Does anybody want something done after I finish?");

        if let Some(on_finish) = self.on_finish.as_ref() {
            on_finish.execute()
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::{cmd::{SimpleCommand, ComplexCommand}, Invoker, Receiver};

    #[test]
    fn test_cmd() {
        let mut invoker = Invoker::default();
        let receiver: Receiver = Receiver::default();

        let simple_cmd= SimpleCommand::new("Say Hi!".into());
        let complex_cmd = ComplexCommand::new(Box::new(receiver), "Send email".into(), "Save report".into());

        invoker.set_on_start(Box::new(simple_cmd));
        invoker.set_on_finish(Box::new(complex_cmd));
        invoker.do_something_important();
    }
}
