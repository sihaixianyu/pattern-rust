use crate::structural::bridge::remote::{AdvancedRemote, BasicRemote, HasMutableDevice, Remote};

use self::device::Device;

pub mod device;
pub mod remote;

pub fn test_device(device: impl Device + Clone) {
    println!("Test basic remote...");
    let mut basic_remote = BasicRemote::new(device.clone());
    basic_remote.power();
    basic_remote.device().print_status();

    println!("Test advanced remote...");
    let mut advanced_remote = AdvancedRemote::new(device.clone());
    advanced_remote.power();
    advanced_remote.mute();
    advanced_remote.device().print_status();
}

#[cfg(test)]
mod tests {
    use super::{device::{TV, Radio}, test_device};

    #[test]
    fn test_bridge() {
        test_device(TV::default());
        test_device(Radio::default());
    }
}
