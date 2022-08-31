use keyboard_query::{DeviceQuery, DeviceState};

use rustris_core::input::{Input, InputSource};

pub struct KeyboardQueryInputSource {
    device_state: DeviceState,
    prev_keys: Vec<u16>,
}

impl KeyboardQueryInputSource {
    pub fn new() -> Self {
        Self {
            device_state: DeviceState::new(),
            prev_keys: vec![],
        }
    }

    fn is_press(&self, keys: &Vec<u16>, key: u16) -> bool {
        !self.prev_keys.contains(&key) && keys.contains(&key)
    }

    fn is_release(&self, keys: &Vec<u16>, key: u16) -> bool {
        self.prev_keys.contains(&key) && !keys.contains(&key)
    }
}

impl InputSource for KeyboardQueryInputSource {
    fn inputs(&mut self) -> Vec<Input> {
        let keys = self.device_state.get_keys();
        let mut inputs = Vec::<Input>::new();
        if keys != self.prev_keys {
            if self.is_press(&keys, 123) {
                inputs.push(Input::LeftPress);
            }
            if self.is_release(&keys, 123) {
                inputs.push(Input::LeftRelease);
            }
            if self.is_press(&keys, 124) {
                inputs.push(Input::RightPress);
            }
            if self.is_release(&keys, 124) {
                inputs.push(Input::RightRelease);
            }
            if self.is_press(&keys, 125) {
                inputs.push(Input::SoftDropPress);
            }
            if self.is_release(&keys, 125) {
                inputs.push(Input::SoftDropRelease);
            }
            if self.is_press(&keys, 126) {
                inputs.push(Input::HardDrop);
            }
            if self.is_press(&keys, 6) {
                inputs.push(Input::RotateLeft);
            }
            if self.is_press(&keys, 7) {
                inputs.push(Input::RotateRight);
            }
            if self.is_press(&keys, 56) {
                inputs.push(Input::Hold);
            }
            if self.is_press(&keys, 36) {
                inputs.push(Input::Pause);
            }
        }
        self.prev_keys = keys;
        inputs
    }
}
