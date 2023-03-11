use super::{specific_target::SpecificTarget, target::Target};

pub struct TargetAdapter {
    target: SpecificTarget,
}

impl TargetAdapter {
    pub fn new(target: SpecificTarget) -> Self {
        Self { target }
    }
}

impl Target for TargetAdapter {
    fn request(&self) -> String {
        self.target.specific_request().chars().rev().collect()
    }
}
