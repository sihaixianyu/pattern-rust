#[derive(Debug)]
pub struct User {
    pub name: String,
    pub age: i32,
}

impl User {
    pub fn new(name: String, age: i32) -> Self {
        User { name, age }
    }
}

pub struct UserCollection {
    users: Vec<User>,
}

impl UserCollection {
    pub fn new(users: Vec<User>) -> Self {
        Self { users }
    }

    pub fn iter(&self) -> UserIterator {
        UserIterator {
            users: &self.users,
            index: 0,
        }
    }
}

pub struct UserIterator<'a> {
    users: &'a Vec<User>,
    index: usize,
}

impl<'a> Iterator for UserIterator<'a> {
    type Item = &'a User;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.users.len() {
            return None;
        }

        let u = &self.users[self.index];
        self.index += 1;

        Some(u)
    }
}
