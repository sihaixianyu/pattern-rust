use super::{
    builder::Builder,
    comp::{CarType, Engine, GPSNavigator, GearType},
};

pub const DEFAULT_FUEL: f64 = 5f64;

#[derive(Debug)]
pub struct Car {
    car_type: CarType,
    gear_type: GearType,
    seat_num: u16,
    engine: Engine,
    gps_navigator: Option<GPSNavigator>,
    fuel: f64,
}

impl Car {
    pub fn new(
        car_type: CarType,
        gear_type: GearType,
        seat_num: u16,
        engine: Engine,
        gps_navigator: Option<GPSNavigator>,
        fuel: f64,
    ) -> Self {
        Self {
            car_type,
            gear_type,
            seat_num,
            engine,
            gps_navigator,
            fuel,
        }
    }
}

impl std::fmt::Display for Car {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{:?}{:?}{:?}{:?}{:?}{:?}",
            self.car_type,
            self.gear_type,
            self.seat_num,
            self.engine,
            self.gps_navigator,
            self.fuel
        )?;

        Ok(())
    }
}

#[derive(Default)]
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
