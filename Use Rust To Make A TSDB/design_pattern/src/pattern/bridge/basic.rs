use crate::pattern::bridge::device::Device;
use crate::pattern::bridge::remote::{HasMuteDevice, Remote};

pub struct BasicRemote<D:Device> {
    device: D
}

impl<D:Device> BasicRemote<D> {
    pub fn new(device: D) -> BasicRemote<D>{
        Self {
            device
        }
    }
}


impl<D: Device> HasMuteDevice<D> for BasicRemote<D>{
    fn mut_device(&mut self) -> &mut D {
        &mut self.device
    }

    fn device(&self) -> &D {
        &self.device
    }
}

impl<D: Device> Remote<D> for BasicRemote<D> {}
