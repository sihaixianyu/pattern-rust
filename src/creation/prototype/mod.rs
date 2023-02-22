#[derive(Clone)]
pub struct Circle {
    pub x: u32,
    pub y: u32,
    pub radius: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let circle = Circle {
            x: 10,
            y: 15,
            radius: 10,
        };

        let mut circle_copy = circle.clone();
        circle_copy.radius = 77;

        println!("Circle: {}, {}, {}", circle.x, circle.y, circle.radius);
        println!("Circle: {}, {}, {}", circle_copy.x, circle_copy.y, circle_copy.radius);
    }
}
