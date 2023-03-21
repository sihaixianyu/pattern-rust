pub mod pizza;
pub mod topping;

#[cfg(test)]
pub mod tests {
    use super::{
        pizza::{Pizza, VeggiePizza},
        topping::{CheeseTopping, TomatoTopping},
    };

    #[test]
    pub fn test_decorator() {
        let pizza = VeggiePizza {};

        let pizza_tomato = TomatoTopping {
            pizza: Box::new(pizza),
        };

        let pizza_cheese_tomato = CheeseTopping {
            pizza: Box::new(pizza_tomato),
        };

        assert_eq!(pizza_cheese_tomato.get_prize(), 32);
    }
}
