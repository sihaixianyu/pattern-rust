use std::sync::Arc;

use super::dress::Dressable;
use super::dress_fcty::{self, DressType};

pub struct Player {
    pub dress: Arc<dyn Dressable>,
    pub player_type: String,
}

impl Player {
    pub fn new(player_type: String, dress_type: DressType) -> Self {
        let dress = dress_fcty::get_dress_type(dress_type).unwrap();

        return Player { player_type, dress };
    }
}

pub struct Game {
    terrorists: Vec<Player>,
    counter_terrorists: Vec<Player>,
}

impl Game {
    pub fn new() -> Self {
        return Self {
            terrorists: Vec::new(),
            counter_terrorists: Vec::new(),
        };
    }

    pub fn add_terrorist(&mut self, dress_type: DressType) {
        let p = Player::new("T".into(), dress_type);
        self.terrorists.push(p)
    }

    pub fn add_counter_terrorist(&mut self, dress_type: DressType) {
        let p = Player::new("CT".into(), dress_type);
        self.counter_terrorists.push(p)
    }
}
