mod device;
mod advanced;
mod basic;
mod radio;
mod remote;
mod tv;

use crate::pattern::bridge::advanced::AdvancedRemote;
use crate::pattern::bridge::basic::BasicRemote;
use crate::pattern::bridge::device::Device;
use crate::pattern::bridge::radio::Radio;
use crate::pattern::bridge::remote::{HasMuteDevice, Remote};
use crate::pattern::bridge::tv::Tv;




fn test_device(device: impl Device) {
    let mut basic_remote = BasicRemote::new(device.clone());
    println!("==========");
    println!("basic remote");
    basic_remote.device().print_status();
    basic_remote.toggle_power();
    basic_remote.volume_up();
    basic_remote.volume_up();
    basic_remote.channel_up();
    basic_remote.device().print_status();
    println!("\n\n");
    let mut advance_remote = AdvancedRemote::new(device.clone());
    println!("==========");
    println!("advance remote");
    advance_remote.device().print_status();
    advance_remote.toggle_power();
    advance_remote.volume_up();
    advance_remote.volume_up();
    advance_remote.channel_up();
    advance_remote.device().print_status();
    advance_remote.mute();
    advance_remote.device().print_status();
    println!("\n\n");
}

#[test]
fn test() {
    test_device(Tv::default());

    test_device(Radio::default());
}





//
// fn main() {
//     test_device(Tv::default());
//     test_device(Radio::default());
// }
//
// fn test_device(device: impl Device + Clone) {
//     println!("Tests with basic remote.");
//     let mut basic_remote = BasicRemote::new(device.clone());
//     basic_remote.power();
//     basic_remote.device().print_status();
//
//     println!("Tests with advanced remote.");
//     let mut advanced_remote = AdvancedRemote::new(device);
//     advanced_remote.power();
//     advanced_remote.mute();
//     advanced_remote.device().print_status();
// }