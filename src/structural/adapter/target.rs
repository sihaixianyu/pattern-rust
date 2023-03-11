pub trait Target {
    fn request(&self) -> String;
}

pub struct OrdinaryTarget;

impl Target for OrdinaryTarget {
    fn request(&self) -> String {
        "ordinary request".into()
    }
}
