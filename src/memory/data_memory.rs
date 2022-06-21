use std::collections::HashMap;

use crate::memory::InnerData;

#[derive(Debug)]
pub struct DataMemory {
    pub data: HashMap<u8, InnerData>,
}

impl DataMemory {
    pub fn new() -> DataMemory {
        DataMemory {
            data: HashMap::new(),
        }
    }

    pub fn get_var_value(&self, idx: u8) -> &InnerData {
        if let Some(value) = self.data.get(&idx) {
            value
        } else {
            &InnerData::INT(0)
        }
    }

    pub fn set_var_value(&mut self, idx: u8, value: InnerData) {
        self.data.insert(idx, value);
    }
}