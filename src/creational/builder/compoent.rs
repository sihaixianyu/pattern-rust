pub enum CarType {
    CityCar,
    SportsCar,
}

pub enum GearType {
    Manual,
    Automatic,
    SemiAutomatic,
}

pub struct Engine {
    pub volume: f64,
    pub mileage: f64,
    pub started: bool,
}

impl Engine {
    pub fn new(volume: f64, mileage: f64) -> Self {
        return Self {
            volume,
            mileage,
            started: false,
        };
    }

    pub fn on(&mut self) {
        self.started = true;
    }

    pub fn off(&mut self) {
        self.started = false;
    }

    pub fn go(&mut self, mileage: f64) -> Result<(), String> {
        if self.started {
            self.mileage += mileage;
            Ok(())
        } else {
            Err("engine must be started first!".to_owned())
        }
    }
}

pub struct GPSNavigator {
    pub route: String,
}

impl GPSNavigator {
    pub fn new(route: String) -> GPSNavigator {
        Self { route }
    }
}
