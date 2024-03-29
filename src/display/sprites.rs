use crate::display::WIDTH;

const _CHAR_0:[u8; 5 * 9] = [
    255, 0, 0, 0, 255,
    0, 255, 255, 255, 0,
    0, 255, 255, 0, 0,
    0, 255, 0, 255, 0,
    0, 255, 0, 255, 0,
    0, 255, 0, 255, 0,
    0, 0, 255, 255, 0,
    0, 255, 255, 255, 0,
    255, 0, 0, 0, 255
];

const _CHAR_1:[u8; 5 * 9] = [
    255, 255, 0, 255, 255,
    0, 0, 0, 255, 255,
    255, 255, 0, 255, 255,
    255, 255, 0, 255, 255,
    255, 255, 0, 255, 255,
    255, 255, 0, 255, 255,
    255, 255, 0, 255, 255,
    255, 255, 0, 255, 255,
    0, 0, 0, 0, 0
];

const _CHAR_2:[u8; 5 * 9] = [
    255, 0, 0, 0, 255,
    0, 255, 255, 255, 0,
    255, 255, 255, 255, 0,
    255, 255, 255, 255, 0,
    255, 255, 0, 0, 255,
    255, 0, 255, 255, 255,
    0, 255, 255, 255, 255,
    0, 255, 255, 255, 255,
    0, 0, 0, 0, 0
];

const _CHAR_3:[u8; 5 * 9] = [
    255, 0, 0, 0, 255,
    0, 255, 255, 255, 0,
    255, 255, 255, 255, 0,
    255, 255, 255, 255, 0,
    255, 0, 0, 0, 255,
    255, 255, 255, 255, 0,
    255, 255, 255, 255, 0,
    0, 255, 255, 255, 0,
    255, 0, 0, 0, 255
];

const _CHAR_4:[u8; 5 * 9] = [
    255, 255, 0, 0, 255,
    255, 0, 255, 0, 255,
    255, 0, 255, 0, 255,
    0, 255, 255, 0, 255,
    0, 255, 255, 0, 255,
    0, 0, 0, 0, 0,
    255, 255, 255, 0, 255,
    255, 255, 255, 0, 255,
    255, 255, 255, 0, 255
];

const _CHAR_5:[u8; 5 * 9] = [
    0, 0, 0, 0, 0,
    0, 255, 255, 255, 255,
    0, 255, 255, 255, 255,
    0, 0, 0, 0, 255,
    255, 255, 255, 255, 0,
    255, 255, 255, 255, 0,
    255, 255, 255, 255, 0,
    0, 255, 255, 255, 0,
    255, 0, 0, 0, 255
];

const _CHAR_6:[u8; 5 * 9] = [
    255, 0, 0, 0, 255,
    0, 255, 255, 255, 0,
    0, 255, 255, 255, 255,
    0, 255, 255, 255, 255,
    0, 0, 0, 0, 255,
    0, 255, 255, 255, 0,
    0, 255, 255, 255, 0,
    0, 255, 255, 255, 0,
    255, 0, 0, 0, 255
];

const _CHAR_7:[u8; 5 * 9] = [
    0, 0, 0, 0, 0,
    0, 255, 255, 255, 0,
    255, 255, 255, 255, 0,
    255, 255, 255, 0, 255,
    255, 255, 0, 255, 255,
    255, 0, 255, 255, 255,
    0, 255, 255, 255, 255,
    0, 255, 255, 255, 255,
    0, 255, 255, 255, 255
];

const _CHAR_8:[u8; 5 * 9] = [
    255, 0, 0, 0, 255,
    0, 255, 255, 255, 0,
    0, 255, 255, 255, 0,
    0, 255, 255, 255, 0,
    255, 0, 0, 0, 255,
    0, 255, 255, 255, 0,
    0, 255, 255, 255, 0,
    0, 255, 255, 255, 0,
    255, 0, 0, 0, 255
];

const _CHAR_9:[u8; 5 * 9] = [
    255, 0, 0, 0, 255,
    0, 255, 255, 255, 0,
    0, 255, 255, 255, 0,
    0, 255, 255, 255, 0,
    255, 0, 0, 0, 0,
    255, 255, 255, 255, 0,
    0, 255, 255, 255, 0,
    0, 255, 255, 255, 0,
    255, 0, 0, 0, 255
];

#[allow(dead_code)]
pub const CHAR_0:(usize, usize, &[u8]) = (
    5,
    9,
    &_CHAR_0
);

#[allow(dead_code)]
pub const CHAR_1:(usize, usize, &[u8]) = (
    5,
    9,
    &_CHAR_1
);

#[allow(dead_code)]
pub const CHAR_2:(usize, usize, &[u8]) = (
    5,
    9,
    &_CHAR_2
);

#[allow(dead_code)]
pub const CHAR_3:(usize, usize, &[u8]) = (
    5,
    9,
    &_CHAR_3
);

#[allow(dead_code)]
pub const CHAR_4:(usize, usize, &[u8]) = (
    5,
    9,
    &_CHAR_4
);

#[allow(dead_code)]
pub const CHAR_5:(usize, usize, &[u8]) = (
    5,
    9,
    &_CHAR_5
);

#[allow(dead_code)]
pub const CHAR_6:(usize, usize, &[u8]) = (
    5,
    9,
    &_CHAR_6
);

#[allow(dead_code)]
pub const CHAR_7:(usize, usize, &[u8]) = (
    5,
    9,
    &_CHAR_7
);

#[allow(dead_code)]
pub const CHAR_8:(usize, usize, &[u8]) = (
    5,
    9,
    &_CHAR_8
);

#[allow(dead_code)]
pub const CHAR_9:(usize, usize, &[u8]) = (
    5,
    9,
    &_CHAR_9
);

#[allow(dead_code)]
pub const CHARS:[(usize, usize, &[u8]); 10] = [
    CHAR_0,
    CHAR_1,
    CHAR_2,
    CHAR_3,
    CHAR_4,
    CHAR_5,
    CHAR_6,
    CHAR_7,
    CHAR_8,
    CHAR_9
];

const _MARKER:[u8; 16 * 11] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 255,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 255,
];

#[allow(dead_code)]
pub const MARKER:(usize, usize, &[u8]) = (
    16,
    11,
    &_MARKER
);

const _CHECK:[u8; 8 * 6] = [
    255, 255, 255, 255, 255, 255, 0, 0,
    255, 255, 255, 255, 255, 0, 0, 255,
    0, 0, 255, 255, 255, 0, 255, 255,
    255, 0, 0, 255, 0, 0, 255, 255,
    255, 255, 0, 0, 0, 255, 255, 255,
    255, 255, 255, 0, 255, 255, 255, 255
];

#[allow(dead_code)]
pub const CHECK:(usize, usize, &[u8]) = (
    8,
    6,
    &_CHECK
);

const _CROSS:[u8; 5 * 7] = [
    0, 255, 255, 255, 0,
    0, 0, 255, 0, 0,
    255, 0, 0, 0, 255,
    255, 255, 0, 255, 255,
    255, 0, 0, 0, 255,
    0, 0, 255, 0, 0,
    0, 255, 255, 255, 0
];

#[allow(dead_code)]
pub const CROSS:(usize, usize, &[u8]) = (
    5,
    7,
    &_CROSS
);

pub fn place_sprite(sprite:&(usize, usize, &[u8]), x:u32, y:u32, invert:bool, image:&mut Vec<u8>) {
    let mut row:usize = 0;
    let (sprite_width, _sprite_height, sprite_data) = sprite;

    let mut data = sprite_data.to_vec();

    if invert {
        for i in &mut data {
            if *i == 0 {
                *i = 255;
            } else {
                *i = 0;
            }
        }
    }

    for (i, _pixel) in sprite.2.iter().enumerate() {
        if i % sprite_width == 0 {
            let origin:usize = ((x + (y * WIDTH)) + (row as u32 * WIDTH)).try_into().unwrap();

            image.splice(origin..(origin + sprite_width), data[(row * sprite_width)..((row * sprite_width) + sprite_width)].iter().cloned());

            row += 1;
        }
    }
}
