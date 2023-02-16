use super::compoent::{CarType, Engine, GPSNavigator, GearType};

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
