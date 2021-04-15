pub struct HiddenInformationContainer<'a> {
    pub data: &'a [u8],
    current_byte_index: usize
}

impl<'a> HiddenInformationContainer<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        HiddenInformationContainer {
            data: data,
            current_byte_index: 0
        }
    }

    pub fn inject_in_angle(&mut self, angle: i8) -> i8 {
        let sub_angle = (angle / 2) * 2;

        let injected_value: i8 = self.data[self.current_byte_index] as i8;
        self.current_byte_index += 1;
        if self.current_byte_index >= self.data.len() {
            self.current_byte_index = 0;
        }

        let angle = sub_angle + injected_value;

        println!("Angle value: {}, injected value: {}", (angle + 3 as i8) as u32, injected_value);

        angle
    }
}