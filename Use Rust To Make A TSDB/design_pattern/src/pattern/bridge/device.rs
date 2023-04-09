
pub trait Device: Clone {
    fn is_enable(&self) -> bool;
    fn enable(&mut self);
    fn disable(&mut self);

    fn get_volume(&self) -> u32;
    fn set_volume(&mut self, volume: u32);

    fn get_channel(&self) -> u32;
    fn set_channel(&mut self, channel: u32);

    fn print_status(&self);
}
