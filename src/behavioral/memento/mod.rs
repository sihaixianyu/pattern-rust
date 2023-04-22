pub trait Memento<T> {
    fn restore(self) -> T;
    fn print(&self);
}

pub struct Originator {
    state: u32,
}

impl Originator {
    pub fn save(&self) -> OriginatorBackup {
        OriginatorBackup(self.state.to_string())
    }
}

pub struct OriginatorBackup(String);

impl Memento<Originator> for OriginatorBackup {
    fn restore(self) -> Originator {
        Originator {
            state: self.0.parse().unwrap(),
        }
    }

    fn print(&self) {
        println!("OriginatorBackup: {}", self.0);
    }
}

#[cfg(test)]
mod tests {
    use super::{Memento, Originator};

    #[test]
    fn test_memento() {
        let mut history = Vec::new();

        let mut originator = Originator { state: 0 };

        originator.state = 1;
        history.push(originator.save());

        originator.state = 2;
        history.push(originator.save());

        originator.state = 3;
        println!("Now state: {}", originator.state);

        for m in history.iter() {
            m.print();
        }

        let orginator = history.pop().unwrap().restore();
        println!("Restore to  state: {}", orginator.state);

        let orginator = history.pop().unwrap().restore();
        println!("Restore to  state: {}", orginator.state);
    }
}
