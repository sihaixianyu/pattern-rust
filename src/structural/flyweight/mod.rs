pub mod dress;
pub mod dress_fcty;
pub mod game;

#[cfg(test)]
mod tests {
    use std::thread;

    use super::{
        dress_fcty::{DressType, DRESS_FACTORY_SINGLETON},
        game::Game,
    };

    #[test]
    pub fn test_flyweight() {
        let handle1 = thread::spawn(|| {
            let mut game1 = Game::new();

            game1.add_terrorist(DressType::Terrorist);
            game1.add_terrorist(DressType::Terrorist);
            game1.add_counter_terrorist(DressType::CounterTerrorist);

            println!("Exit handle1");
        });

        let handle2 = thread::spawn(|| {
            let mut game2 = Game::new();

            game2.add_terrorist(DressType::Terrorist);
            game2.add_terrorist(DressType::Terrorist);
            game2.add_counter_terrorist(DressType::CounterTerrorist);
            game2.add_counter_terrorist(DressType::CounterTerrorist);

            println!("Exit handle2");
        });

        if let Err(e) = handle1.join() {
            panic!("{:?}", e)
        }
        if let Err(e) = handle2.join() {
            panic!("{:?}", e)
        }

        let fcty = DRESS_FACTORY_SINGLETON.lock().unwrap();

        for (dress_type, dress) in fcty.iter() {
            println!("dress_type {:?}, dress {}", dress_type, dress.get_color())
        }
    }
}
