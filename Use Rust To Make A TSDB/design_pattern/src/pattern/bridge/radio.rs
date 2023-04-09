use crate::pattern::bridge::device::Device;

#[derive(Default, Clone)]
pub struct Radio {
    // 开机还是关机
    on: bool,
    volume: u32,
    channel: u32,
}


impl Device for Radio{
    fn is_enable(&self) -> bool {
        self.on
    }

    fn enable(&mut self) {
        self.on = true;
        println!("turn on");
    }

    fn disable(&mut self) {
        self.on = false;
        println!("turn off");
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
