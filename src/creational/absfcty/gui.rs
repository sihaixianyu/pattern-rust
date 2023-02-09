// Abstract trait defined using generics.
pub trait GuiFactory {
    type B: Button;
    type C: Checkbox;

    fn create_button(&self) -> Self::B;
    fn create_checkbox(&self) -> Self::C;
}

// Abstract trait defined using `Box` pointer.
pub trait GuiFactoryDynamic {
    fn create_button(&self) -> Box<dyn Button>;
    fn create_checkbox(&self) -> Box<dyn Checkbox>;
}

pub trait Button {
    fn press(&self);
}

pub trait Checkbox {
    fn switch(&self);
}