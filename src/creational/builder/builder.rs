use super::{
    comp::{CarType, Engine, GPSNavigator, GearType},
};

pub trait Builder {
    type OutputType;
    fn set_car_type(&mut self, car_type: CarType);
    fn set_gear_type(&mut self, gear_type: GearType);
    fn set_seat_num(&mut self, seats: u16);
    fn set_engine(&mut self, engine: Engine);
    fn set_gsp_navigator(&mut self, gps_navigator: GPSNavigator);
    fn build(self) -> Self::OutputType;
}
