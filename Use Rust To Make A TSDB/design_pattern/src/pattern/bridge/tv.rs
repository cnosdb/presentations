use crate::pattern::bridge::device::Device;
#[derive(Default, Clone)]
pub struct Tv {
    on: bool,
    volume: u32,
    channel: u32,
}

impl Device for Tv {
    fn is_enable(&self) -> bool {
        self.on
    }

    fn enable(&mut self) {
        self.on = true;
    }

    fn disable(&mut self) {
        self.on = false;
    }

    fn get_volume(&self) -> u32 {
        self.volume
    }

    fn set_volume(&mut self, volume: u32) {
        self.volume = volume;
    }

    fn get_channel(&self) -> u32 {
        self.channel
    }

    fn set_channel(&mut self, channel: u32) {
        self.channel = channel;
    }
    fn print_status(&self) {
        println!("===========");
        println!("on: {}", self.on);
        println!("volume: {}", self.volume);
        println!("channel: {}", self.channel);
        println!("===========");
    }
}
