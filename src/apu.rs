pub struct Apu {}

impl Apu {
    pub fn new() -> Apu {
        Apu {}
    }

    pub fn read(&mut self, _address: u16) -> u8 {
        0
    }

    pub fn write(&mut self, _address: u16, _data: u8) {}
}
