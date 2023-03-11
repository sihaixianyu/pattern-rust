pub mod specific_target;
pub mod target;
pub mod target_adapter;

use self::target::Target;

pub fn call(target: impl Target) {
    println!("'{}'", target.request());
}

#[cfg(test)]
mod tests {

    use super::{
        call, specific_target::SpecificTarget, target::OrdinaryTarget,
        target_adapter::TargetAdapter,
    };

    #[test]
    pub fn test_adapter() {
        let tar = OrdinaryTarget;

        print!("A compatible target can be directly called: ");
        call(tar);

        let adptee = SpecificTarget;
        println!(
            "Adaptee is incompatible with client: '{}'",
            adptee.specific_request()
        );

        let adapter = TargetAdapter::new(adptee);

        print!("But with a adapter client, you can call its method: ");
        call(adapter);
    }
}
