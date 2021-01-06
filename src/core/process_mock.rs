pub struct Process {}

impl Process {
    pub fn find(_exe_file: &str) -> Option<Process> {
        Some(Process {})
    }

    pub fn path(&self) -> String {
        "".into()
    }

    pub fn read_u32(&self, _address: u32) -> u32 {
        0
    }

    pub fn read_i32(&self, _address: u32) -> i32 {
        0
    }

    // pub fn read_u16(&self, _address: u32) -> u16 {
    //     0
    // }

    pub fn read_u8(&self, _address: u32) -> u8 {
        0
    }

    pub fn read_f32(&self, _address: u32) -> f32 {
        0.0
    }

    pub fn write_u8(&self, _address: u32, _value: u8) {}

    pub fn write_i32(&self, _address: u32, _value: i32) {}

    pub fn write_f32(&self, _address: u32, _value: f32) {}

    // pub fn write(&self, address: u32, buf: &[u8]) {}
}

impl Drop for Process {
    fn drop(&mut self) {}
}
