use super::mediator::Mediator;

pub trait Train {
    fn name(&self) -> &String;
    fn arrive(&mut self, mediator: &mut dyn Mediator);
    fn depart(&mut self, mediator: &mut dyn Mediator);
}

pub struct PassengerTrain {
    name: String,
}

impl PassengerTrain {
    pub fn new(name: &str) -> Self {
        Self { name: name.into() }
    }
}

impl Train for PassengerTrain {
    fn name(&self) -> &String {
        &self.name
    }

    fn arrive(&mut self, mediator: &mut dyn Mediator) {
        if !mediator.notify_arrival(&self.name) {
            println!("Passenger train: {}: Arrival blocked, waiting", self.name);
            return;
        }

        println!("Passenger train: {}: Arrived", self.name);
    }

    fn depart(&mut self, mediator: &mut dyn Mediator) {
        println!("Passenger train {}: Leaving", self.name);
        mediator.notify_depart(&self.name);
    }
}

pub struct FreightTrain {
    name: String,
}

impl FreightTrain {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
        }
    }
}

impl Train for FreightTrain {
    fn name(&self) -> &String {
        &self.name
    }

    fn arrive(&mut self, mediator: &mut dyn Mediator) {
        if !mediator.notify_arrival(&self.name) {
            println!("Freight train: {}: Arrival blocked, waiting", self.name);
            return;
        }

        println!("Freight train: {}: Arrived", self.name);
    }

    fn depart(&mut self, mediator: &mut dyn Mediator) {
        println!("Freight train {}: Leaving", self.name);
        mediator.notify_depart(&self.name);
    }
}
