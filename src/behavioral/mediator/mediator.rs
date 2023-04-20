use std::collections::{HashMap, VecDeque};

use super::train::Train;

pub trait Mediator {
    fn notify_arrival(&mut self, train_name: &str) -> bool;
    fn notify_depart(&mut self, train_name: &str);
}

#[derive(Default)]
pub struct TrainStation {
    trains: HashMap<String, Box<dyn Train>>,
    waitting_queue: VecDeque<String>,
    on_platform: Option<String>,
}

impl TrainStation {
    pub fn accpet(&mut self, mut train: impl Train + 'static) {
        if self.trains.contains_key(train.name()) {
            println!("{} has already arrived", train.name());
            return;
        }

        train.arrive(self);
        self.trains.insert(train.name().clone(), Box::new(train));
    }

    pub fn depart(&mut self, train_name: &str) {
        let train  = self.trains.remove(train_name);
        if let Some(mut train) = train {
            train.depart(self);
        } else {
             println!("'{}' is not on the station", train_name);
        }
    }
}

impl Mediator for TrainStation {
    fn notify_arrival(&mut self, train_name: &str) -> bool {
        if let Some(_) = &self.on_platform {
            self.waitting_queue.push_back(train_name.into());
            return false;
        }

        self.on_platform = Some(train_name.into());
        true
    }

    fn notify_depart(&mut self, train_name: &str) {
        let Some(curr_train) = &self.on_platform else {
            panic!("No train in station!");
        };

        if train_name != curr_train {
            panic!(
                "No matched train in station! => on platform: {}, ask for leave: {}",
                curr_train, train_name
            );
        }

        self.on_platform = None;
        self.trains.remove(train_name);

        if let Some(next_train_name) = self.waitting_queue.pop_front() {
            let mut next_train = self.trains.remove(&next_train_name).unwrap();
            next_train.arrive(self);
            self.trains.insert(next_train_name.clone(), next_train);
            self.on_platform = Some(next_train_name);
        }
    }
}
