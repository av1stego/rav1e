use std::fmt::Debug;

pub struct HiddenInformationContainer<> {
    pub data: Vec<u8>,
    enabled: bool,
    current_byte_index: usize
}

impl Default for HiddenInformationContainer {
    fn default() -> Self {
        Self {
            data: vec![],
            enabled: false,
            current_byte_index: 0
        }
    }
}

impl Clone for HiddenInformationContainer {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            enabled: self.enabled,
            current_byte_index: self.current_byte_index
        }
    }
}

impl Debug for HiddenInformationContainer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}) {:?}", self.current_byte_index, self.data)
    }
}

impl HiddenInformationContainer {
    pub fn new(data: Vec<u8>) -> Self {
        HiddenInformationContainer {
            data: data,
            enabled: false,
            current_byte_index: 0
        }
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn inject_in_angle(&mut self, angle: u32) -> u32 {
        if !self.enabled {
            return angle;
        }

        if angle == 6 {
            // println!("Angle is 6, skipping space");
            println!("{}", angle);
            return angle;
        }

        let sub_angle = (angle / 2) * 2;

        /*let injected_value: i8 = self.data[self.current_byte_index] as i8;
        self.current_byte_index += 1;
        if self.current_byte_index >= self.data.len() {
            self.current_byte_index = 0;
        }*/
        let injected_value = 1;

        let new_angle = sub_angle + injected_value;

        // println!("Angle: {}, new angle: {}, injected value: {}", angle, new_angle, injected_value);
        println!("{}", new_angle);

        new_angle
    }
}