pub trait TemplateMethod {
    fn template_method(&self) {
        self.base_operation1();
        self.required_operations1();
        self.base_operation2();
        self.hook1();
        self.required_operations2();
        self.base_operation3();
        self.hook2();
    }

    fn base_operation1(&self) {
        println!("TemplateMethod says: I am doing the bulk of the work");
    }

    fn base_operation2(&self) {
        println!("TemplateMethod says: But I let subclasses override some operations");
    }

    fn base_operation3(&self) {
        println!("TemplateMethod says: But I am doing the bulk of the work anyway");
    }

    fn hook1(&self) {}
    fn hook2(&self) {}

    fn required_operations1(&self);
    fn required_operations2(&self);
}

pub struct ConcreteStruct1;

impl TemplateMethod for ConcreteStruct1 {
    fn required_operations1(&self) {
        println!("ConcreteStruct1 says: Implemented Operation1")
    }

    fn required_operations2(&self) {
        println!("ConcreteStruct1 says: Implemented Operation2")
    }
}

pub struct ConcreteStruct2;

impl TemplateMethod for ConcreteStruct2 {
    fn required_operations1(&self) {
        println!("ConcreteStruct2 says: Implemented Operation1")
    }

    fn required_operations2(&self) {
        println!("ConcreteStruct2 says: Implemented Operation2")
    }
}

#[cfg(test)]
mod tests {
    use super::{ConcreteStruct1, ConcreteStruct2, TemplateMethod};

    #[test]
    fn test_template() {
        println!("Same client code can work with different concrete implementations:");
        ConcreteStruct1.template_method();

        println!("Same client code can work with different concrete implementations:");
        ConcreteStruct2.template_method();
    }
}
