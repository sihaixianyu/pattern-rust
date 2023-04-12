use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use once_cell::sync::Lazy;

use super::dress::{CounterTerroristDress, Dressable, TerroristDress};

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub enum DressType {
    Terrorist,
    CounterTerrorist,
}

type DressFactory = HashMap<DressType, Arc<dyn Dressable>>;

pub static DRESS_FACTORY_SINGLETON: Lazy<Mutex<DressFactory>> =
    Lazy::new(|| Mutex::new(DressFactory::new()));

pub fn get_dress_type(dress_type: DressType) -> Result<Arc<dyn Dressable>, String> {
    let mut fcty = DRESS_FACTORY_SINGLETON.lock().unwrap();

    if let Some(d) = fcty.get(&dress_type) {
        return Ok(d.clone());
    }

    if dress_type == DressType::Terrorist {
        let dress = Arc::new(TerroristDress::new());
        fcty.insert(dress_type, dress.clone());
        return Ok(dress);
    }

    if dress_type == DressType::CounterTerrorist {
        let dress = Arc::new(CounterTerroristDress::new());
        fcty.insert(dress_type, dress.clone());
        return Ok(dress);
    }

    Err("unsupported dress type!".to_string())
}
