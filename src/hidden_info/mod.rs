use std::fmt::Debug;

pub struct HiddenInformationContainer<> {
    pub data: Vec<u8>,

    pre_encoding: bool,
    booked_byte_index: usize,
    booked_bit_index: usize,

    final_encoding: bool,
    current_byte_index: usize,
    current_bit_index: usize,
}

impl Default for HiddenInformationContainer {
    fn default() -> Self {
        Self {
            data: vec![],

            pre_encoding: false,
            booked_byte_index: 0,
            booked_bit_index: 0,

            final_encoding: false,

            current_byte_index: 0,
            current_bit_index: 0,
        }
    }
}

impl Clone for HiddenInformationContainer {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            pre_encoding: self.pre_encoding,
            booked_byte_index: self.booked_byte_index,
            booked_bit_index: self.booked_bit_index,
            final_encoding: self.final_encoding,
            current_byte_index: self.current_byte_index,
            current_bit_index: self.current_bit_index
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
            pre_encoding: false,
            booked_byte_index: 0,
            booked_bit_index: 0,
            final_encoding: false,
            current_byte_index: 0,
            current_bit_index: 0,
        }
    }

    pub fn new_from_str(string: String) -> Self {
        let mut str_bytes = string.into_bytes();
        str_bytes.push(0b0);

        HiddenInformationContainer {
            data: str_bytes,
            pre_encoding: false,
            booked_byte_index: 0,
            booked_bit_index: 0,
            final_encoding: false,
            current_byte_index: 0,
            current_bit_index: 0,
        }
    }

    pub fn start_pre_encoding(&mut self) {
        self.booked_byte_index = self.current_byte_index;
        self.booked_bit_index = self.current_bit_index;
        self.pre_encoding = true;
    }

    pub fn stop_pre_encoding(&mut self) {
        assert!(self.pre_encoding);

        self.current_byte_index = self.booked_byte_index;
        self.current_bit_index = self.booked_bit_index;

        self.pre_encoding = false;
    }

    pub fn start_final_encoding(&mut self) {
        self.final_encoding = true;
    }

    pub fn stop_final_encoding(&mut self) {
        self.final_encoding = false;
    }

    pub fn is_enabled(&self) -> bool {
        self.pre_encoding || self.final_encoding
    }

    pub fn is_done(&self) -> bool {
        self.current_byte_index >= self.data.len()
    }

    pub fn inject_in_angle(&mut self, angle: u32) -> u32 {
        if !self.is_enabled() {
            return angle;
        }

        if angle == 6 {
            // println!("Angle is 6, skipping space");
            return angle;
        }

        if self.is_done() {
            if self.final_encoding {
                println!("All data were trasmitted");
            }

            return angle;
        }

        let sub_angle = (angle / 2) * 2;

        let injected_value: u32 = ((self.data[self.current_byte_index] as u32) >> self.current_bit_index) & 1;
        self.current_bit_index += 1;
        if self.current_bit_index == 8 {
            self.current_byte_index += 1;
            self.current_bit_index = 0;
        }

        let new_angle = sub_angle + injected_value;

        if self.final_encoding {
            println!("Angle: {}, new angle: {}, injected value => {}", angle, new_angle, injected_value);
        }

        new_angle
    }
}