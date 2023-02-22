pub mod builder;
pub mod car;
pub mod car_manual;
pub mod comp;
pub mod director;

#[cfg(test)]
mod tests {
    use crate::creation::builder::car_manual::ManualBuilder;

    use super::{builder::Builder, car::CarBuilder, director::Director};

    #[test]
    fn test_builder() {
        let mut car_builder = CarBuilder::default();
        Director::build_sports_car(&mut car_builder);

        let car = car_builder.build();
        println!("car build:\n{}", car);

        let mut manual_build = ManualBuilder::default();
        Director::build_city_car(&mut manual_build);

        let manual = manual_build.build();
        println!("manual build:\n{}", manual);
    }
}
