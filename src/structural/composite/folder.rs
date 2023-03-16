use super::component::Component;

pub struct Folder {
    name: &'static str,
    components: Vec<Box<dyn Component>>,
}

impl Folder {
    pub fn new(name: &'static str) -> Self {
        Self {
            name,
            components: Vec::new(),
        }
    }

    pub fn add(&mut self, component: impl Component + 'static) {
        self.components.push(Box::new(component));
    }
}

impl Component for Folder {
    fn search(&self, keyword: &str) {
        println!(
            "Searching recursively for keyword {} in folder {}",
            keyword, self.name
        );

        for component in &self.components {
            component.search(keyword);
        }
    }
}
