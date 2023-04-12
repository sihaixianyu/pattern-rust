pub trait Dressable: Send + Sync {
    fn get_color(&self) -> String;
}

#[derive(Debug, Clone)]
pub struct TerroristDress {
    color: String,
}

impl TerroristDress {
    pub fn new() -> Self {
        TerroristDress {
            color: "red".into(),
        }
    }
}

impl Dressable for TerroristDress {
    fn get_color(&self) -> String {
        self.color.clone()
    }
}

#[derive(Debug, Clone)]
pub struct CounterTerroristDress {
    color: String,
}

impl CounterTerroristDress {
    pub fn new() -> Self {
        CounterTerroristDress {
            color: "green".into(),
        }
    }
}

impl Dressable for CounterTerroristDress {
    fn get_color(&self) -> String {
        self.color.clone()
    }
}
