use crate::pattern::bridge::device::Device;
use crate::pattern::bridge::remote::{HasMuteDevice, Remote};

pub struct AdvancedRemote<D: Device> {
    device: D
}


impl<D:Device> AdvancedRemote<D> {
    pub fn new(device: D) -> AdvancedRemote<D>{
        Self {
            device
        }
    }

    pub fn mute(&mut self) {
        self.device.set_volume(0)
    }

}

impl<D: Device> HasMuteDevice<D> for AdvancedRemote<D>{
    fn mut_device(&mut self) -> &mut D {
        &mut self.device
    }

    fn device(&self) -> &D {
        &self.device
    }
}

impl<D: Device> Remote<D> for AdvancedRemote<D> {}

