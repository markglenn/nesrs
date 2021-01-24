const FRAME_WIDTH: usize = 256;
const FRAME_HEIGHT: usize = 240;

const FRAME_SIZE: usize = FRAME_WIDTH * FRAME_HEIGHT * 3;

pub struct Frame {
    pub data: [u8; FRAME_SIZE],
}

impl Frame {
    pub fn new() -> Self {
        Frame {
            data: [0; FRAME_SIZE],
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, rgb: (u8, u8, u8)) {
        let base = y * 3 * FRAME_WIDTH + x * 3;
        if base + 2 < self.data.len() {
            self.data[base] = rgb.0;
            self.data[base + 1] = rgb.1;
            self.data[base + 2] = rgb.2;
        }
    }

    pub fn clear(&mut self) {
        for elem in self.data.iter_mut() {
            *elem = 0;
        }
    }
}
