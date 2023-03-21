pub trait Pizza {
    fn get_prize(&self) -> i32;
}

pub struct VeggiePizza;

impl Pizza for VeggiePizza {
    fn get_prize(&self) -> i32 {
        15
    }
}
