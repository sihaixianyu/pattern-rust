type RouteStrategy = fn(from: &str, to: &str);

pub fn walking_strategy(from: &str, to: &str) {
    println!("Walking route from {} to {}: 4km 30min", from, to);
}

pub fn public_transport_strategy(from: &str, to: &str) {
    println!("Public transport from {} to {}: 4km 30min", from, to);
}

pub struct Navigator {
    route_strategy: RouteStrategy,
}

impl Navigator {
    pub fn new(route_strategy: RouteStrategy) -> Self {
        Self { route_strategy }
    }

    pub fn route(&self, from: &str, to: &str) {
        (self.route_strategy)(from, to);
    }
}
