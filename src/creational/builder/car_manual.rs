use super::{comp::{CarType, Engine, GPSNavigator, GearType}, builder::Builder};

pub struct Manual {
    car_type: CarType,
    gear_type: GearType,
    engine: Engine,
    seat_num: u16,
    gps_navigator: Option<GPSNavigator>,
}

impl Manual {
    pub fn new(
        car_type: CarType,
        gear_type: GearType,
        seat_num: u16,
        engine: Engine,
        gps_navigator: Option<GPSNavigator>,
    ) -> Self {
        Self {
            car_type,
            gear_type,
            seat_num,
            engine,
            gps_navigator,
        }
    }
}

impl std::fmt::Display for Manual {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "type of car: {:?}", self.car_type)?;
        writeln!(f, "nums of set: {:?}", self.seat_num)?;
        writeln!(
            f,
            "Engine: volume - {}; mileage - {}",
            self.engine.volume, self.engine.mileage
        )?;
        writeln!(f, "gear type: {:?}", self.gear_type)?;

        match self.gps_navigator {
            Some(_) => writeln!(f, "gps navigator: functional")?,
            None => writeln!(f, "gps navigator: n/a")?,
        }

        Ok(())
    }
}

#[derive(Default)]
pub struct ManualBuilder {
    car_type: Option<CarType>,
    gear_type: Option<GearType>,
    engine: Option<Engine>,
    seat_num: Option<u16>,
    gps_navigator: Option<GPSNavigator>,
}

impl Builder for ManualBuilder {
    type OutputType = Manual;

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
        Manual::new(
            self.car_type.expect("please set a car type"),
            self.gear_type.expect("please set a gear type"),
            self.seat_num.expect("please set a seat number"),
            self.engine.expect("please set a engine type"),
            self.gps_navigator,
        )
    }
}
