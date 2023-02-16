use super::{
    car::Car,
    compoent::{CarType, Engine, GPSNavigator, GearType},
};

pub const DEFAULT_FUEL: f64 = 5f64;

pub trait Builder {
    type OutputType;
    fn set_car_type(&mut self, car_type: CarType);
    fn set_gear_type(&mut self, gear_type: GearType);
    fn set_seat_num(&mut self, seats: u16);
    fn set_engine(&mut self, engine: Engine);
    fn set_gsp_navigator(&mut self, gps_navigator: GPSNavigator);
    fn build(self) -> Self::OutputType;
}

pub struct CarBuilder {
    car_type: Option<CarType>,
    gear_type: Option<GearType>,
    seat_num: Option<u16>,
    engine: Option<Engine>,
    gps_navigator: Option<GPSNavigator>,
}

impl Builder for CarBuilder {
    type OutputType = Car;

    fn set_car_type(&mut self, car_type: CarType) {
        self.car_type = Some(car_type);
    }

    fn set_gear_type(&mut self, gear_type: GearType) {
        self.gear_type = Some(gear_type);
    }

    fn set_seat_num(&mut self, seat_num: u16) {
        self.seat_num = Some(seat_num);
    }

    fn set_engine(&mut self, engine: Engine) {
        self.engine = Some(engine);
    }

    fn set_gsp_navigator(&mut self, gps_navigator: GPSNavigator) {
        self.gps_navigator = Some(gps_navigator);
    }

    fn build(self) -> Self::OutputType {
        Car::new(
            self.car_type.expect("please set a car type"),
            self.gear_type.expect("please set a gear type"),
            self.seat_num.expect("please set a seat number"),
            self.engine.expect("please set a engine type"),
            self.gps_navigator,
            DEFAULT_FUEL,
        )
    }
}
