pub mod user;

#[cfg(test)]
mod tests {
    use super::user::{User, UserCollection};

    #[test]
    fn test_iterator_pattern() {
        let users = UserCollection::new(vec![
            User::new("Amier".into(), 25),
            User::new("Bruce".into(), 25),
            User::new("Cindy".into(), 25),
        ]);

        for u in users.iter() {
            println!("{:?}", u);
        }
    }
}
