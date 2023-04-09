use crate::pattern::bridge::device::Device;

pub trait HasMuteDevice<D: Device> {
    fn mut_device(&mut self) -> &mut D;
    fn device(&self) -> &D;
}


pub trait Remote<D: Device>: HasMuteDevice<D> {
    fn toggle_power(&mut self) {
        if self.device().is_enable() {
            self.mut_device().disable();
        } else {
            self.mut_device().enable();
        }
    }

    fn volume_down(&mut self) {
        let volume = self.device().get_volume();
        self.mut_device().set_volume(volume - 1);
    }
    fn volume_up(&mut self) {
        let volume = self.device().get_volume();
        self.mut_device().set_volume(volume + 1);
    }
    fn channel_down(&mut self) {
        let channel = self.device().get_channel();
        self.mut_device().set_channel(channel - 1);
    }
    fn channel_up(&mut self) {
        let channel = self.device().get_channel();
        self.mut_device().set_channel(channel + 1);
    }
}
