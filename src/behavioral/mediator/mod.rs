pub mod mediator;
pub mod train;

#[cfg(test)]
mod tests {
    use super::{train::{PassengerTrain, FreightTrain}, mediator::TrainStation};

    #[test]
    fn test_mediator_pattern() {
        let train1 = PassengerTrain::new("Train1");
        let train2 = FreightTrain::new("Train2");

        let mut station = TrainStation::default();
        station.accpet(train1);
        station.accpet(train2);

        station.depart("Train1");
        station.depart("Train2");
        station.depart("Train3");
    }
}
