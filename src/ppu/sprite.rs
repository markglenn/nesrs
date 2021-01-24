use bitfield::bitfield;

bitfield! {
    pub struct SpriteAttribute(u8);

    impl Debug;

    pub palette, _  : 1, 0;
    pub priority, _ : 5;
    pub flip_x, _   : 6;
    pub flip_y, _   : 7;
}

pub struct Sprite {
    pub x: u8,
    pub y: u8,
    pub index: u8,
    pub attributes: SpriteAttribute,
}

impl Sprite {
    pub fn new(data: &[u8]) -> Self {
        Sprite {
            y: data[0],
            index: data[1],
            attributes: SpriteAttribute(data[2]),
            x: data[3],
        }
    }
}
