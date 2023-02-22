use super::{
    builder::Builder,
    comp::{CarType, Engine, GPSNavigator, GearType},
};

pub struct Director;

impl Director {
    pub fn build_city_car(builder: &mut impl Builder) {
        builder.set_car_type(CarType::CityCar);
        builder.set_seat_num(4);
        builder.set_engine(Engine::new(2.5, 0.0));
        builder.set_gear_type(GearType::Automatic);
        builder.set_gsp_navigator(GPSNavigator::new())
    }

    pub fn build_sports_car(builder: &mut impl Builder) {
        builder.set_car_type(CarType::SportsCar);
        builder.set_seat_num(2);
        builder.set_engine(Engine::new(3.0, 0.0));
        builder.set_gear_type(GearType::Manual);
        builder.set_gsp_navigator(GPSNavigator::new())
    }
}
