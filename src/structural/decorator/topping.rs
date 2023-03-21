use super::pizza::Pizza;

pub struct TomatoTopping {
    pub pizza: Box<dyn Pizza>,
}

impl Pizza for TomatoTopping {
    fn get_prize(&self) -> i32 {
        self.pizza.get_prize() + 7
    }
}

pub struct CheeseTopping {
    pub pizza: Box<dyn Pizza>,
}

impl Pizza for CheeseTopping {
    fn get_prize(&self) -> i32 {
        self.pizza.get_prize() + 10
    }
}
