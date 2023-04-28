pub mod fn_impl;
pub mod trait_impl;

#[cfg(test)]
mod tests {
    use super::{
        fn_impl::{public_transport_strategy, walking_strategy, self},
        trait_impl::{PublicTransportStrategy, WalkingStrategy, self},
    };

    #[test]
    fn test_fn_impl() {
        let nav = fn_impl::Navigator::new(walking_strategy);
        nav.route("Home", "Club");
        nav.route("Club", "Work");

        let nav = fn_impl::Navigator::new(public_transport_strategy);
        nav.route("Home", "Club");
        nav.route("Club", "Work");

        let nav = fn_impl::Navigator::new(|from, to| println!("Specific route from {} to {}", from, to));
        nav.route("Home", "Club");
        nav.route("Club", "Work");
    }

    #[test]
    fn test_trait_impl() {
        let nav = trait_impl::Navigator::new(WalkingStrategy);
        nav.route("Home", "Club");
        nav.route("Club", "Work");

        let nav = trait_impl::Navigator::new(PublicTransportStrategy);
        nav.route("Home", "Club");
        nav.route("Club", "Work");
    }
}
