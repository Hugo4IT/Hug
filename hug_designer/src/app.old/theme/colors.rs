//! This is the OpenColor scheme (https://yeun.github.io/open-color)

use iced::Color;

#[allow(dead_code)]
pub const GRAY: [Color; 10] = [
    Color::from_rgb(
        0xF8 as f32 / 255.0f32,
        0xF9 as f32 / 255.0f32,
        0xFA as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0xF1 as f32 / 255.0f32,
        0xF3 as f32 / 255.0f32,
        0xF5 as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0xE9 as f32 / 255.0f32,
        0xEC as f32 / 255.0f32,
        0xEF as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0xDE as f32 / 255.0f32,
        0xE2 as f32 / 255.0f32,
        0xE6 as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0xCE as f32 / 255.0f32,
        0xD4 as f32 / 255.0f32,
        0xDA as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0xAD as f32 / 255.0f32,
        0xB5 as f32 / 255.0f32,
        0xBD as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0x86 as f32 / 255.0f32,
        0x8E as f32 / 255.0f32,
        0x96 as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0x49 as f32 / 255.0f32,
        0x50 as f32 / 255.0f32,
        0x57 as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0x34 as f32 / 255.0f32,
        0x3A as f32 / 255.0f32,
        0x40 as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0x21 as f32 / 255.0f32,
        0x25 as f32 / 255.0f32,
        0x29 as f32 / 255.0f32,
    ),
];

#[allow(dead_code)]
pub const RED: [Color; 10] = [
    Color::from_rgb(
        0xFF as f32 / 255.0f32,
        0xF5 as f32 / 255.0f32,
        0xF5 as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0xFF as f32 / 255.0f32,
        0xE3 as f32 / 255.0f32,
        0xE3 as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0xFF as f32 / 255.0f32,
        0xC9 as f32 / 255.0f32,
        0xC9 as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0xFF as f32 / 255.0f32,
        0xA8 as f32 / 255.0f32,
        0xA8 as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0xFF as f32 / 255.0f32,
        0x87 as f32 / 255.0f32,
        0x87 as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0xFF as f32 / 255.0f32,
        0x6B as f32 / 255.0f32,
        0x6B as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0xFA as f32 / 255.0f32,
        0x52 as f32 / 255.0f32,
        0x52 as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0xF0 as f32 / 255.0f32,
        0x3E as f32 / 255.0f32,
        0x3E as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0xE0 as f32 / 255.0f32,
        0x31 as f32 / 255.0f32,
        0x31 as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0xC9 as f32 / 255.0f32,
        0x2A as f32 / 255.0f32,
        0x2A as f32 / 255.0f32,
    ),
];

#[allow(dead_code)]
pub const PINK: [Color; 10] = [
    Color::from_rgb(
        0xFF as f32 / 255.0f32,
        0xF0 as f32 / 255.0f32,
        0xF6 as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0xFF as f32 / 255.0f32,
        0xDE as f32 / 255.0f32,
        0xEB as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0xFC as f32 / 255.0f32,
        0xC2 as f32 / 255.0f32,
        0xD7 as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0xFA as f32 / 255.0f32,
        0xA2 as f32 / 255.0f32,
        0xC1 as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0xF7 as f32 / 255.0f32,
        0x83 as f32 / 255.0f32,
        0xAC as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0xF0 as f32 / 255.0f32,
        0x65 as f32 / 255.0f32,
        0x95 as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0xE6 as f32 / 255.0f32,
        0x49 as f32 / 255.0f32,
        0x80 as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0xD6 as f32 / 255.0f32,
        0x33 as f32 / 255.0f32,
        0x6C as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0xC2 as f32 / 255.0f32,
        0x25 as f32 / 255.0f32,
        0x5C as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0xA6 as f32 / 255.0f32,
        0x1E as f32 / 255.0f32,
        0x4D as f32 / 255.0f32,
    ),
];

#[allow(dead_code)]
pub const GRAPE: [Color; 10] = [
    Color::from_rgb(
        0xF8 as f32 / 255.0f32,
        0xF0 as f32 / 255.0f32,
        0xFC as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0xF3 as f32 / 255.0f32,
        0xD9 as f32 / 255.0f32,
        0xFA as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0xEE as f32 / 255.0f32,
        0xBE as f32 / 255.0f32,
        0xFA as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0xE5 as f32 / 255.0f32,
        0x99 as f32 / 255.0f32,
        0xF7 as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0xDA as f32 / 255.0f32,
        0x77 as f32 / 255.0f32,
        0xF2 as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0xCC as f32 / 255.0f32,
        0x5D as f32 / 255.0f32,
        0xE8 as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0xBE as f32 / 255.0f32,
        0x4B as f32 / 255.0f32,
        0xDB as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0xAE as f32 / 255.0f32,
        0x3E as f32 / 255.0f32,
        0xC9 as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0x9C as f32 / 255.0f32,
        0x36 as f32 / 255.0f32,
        0xB5 as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0x86 as f32 / 255.0f32,
        0x2E as f32 / 255.0f32,
        0x9C as f32 / 255.0f32,
    ),
];

#[allow(dead_code)]
pub const VIOLET: [Color; 10] = [
    Color::from_rgb(
        0xF3 as f32 / 255.0f32,
        0xF0 as f32 / 255.0f32,
        0xFF as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0xE5 as f32 / 255.0f32,
        0xDB as f32 / 255.0f32,
        0xFF as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0xD0 as f32 / 255.0f32,
        0xBF as f32 / 255.0f32,
        0xFF as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0xB1 as f32 / 255.0f32,
        0x97 as f32 / 255.0f32,
        0xFC as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0x97 as f32 / 255.0f32,
        0x75 as f32 / 255.0f32,
        0xFA as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0x84 as f32 / 255.0f32,
        0x5E as f32 / 255.0f32,
        0xF7 as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0x79 as f32 / 255.0f32,
        0x50 as f32 / 255.0f32,
        0xF2 as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0x70 as f32 / 255.0f32,
        0x48 as f32 / 255.0f32,
        0xE8 as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0x67 as f32 / 255.0f32,
        0x41 as f32 / 255.0f32,
        0xD9 as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0x5F as f32 / 255.0f32,
        0x3D as f32 / 255.0f32,
        0xC4 as f32 / 255.0f32,
    ),
];

#[allow(dead_code)]
pub const INDIGO: [Color; 10] = [
    Color::from_rgb(
        0xED as f32 / 255.0f32,
        0xF2 as f32 / 255.0f32,
        0xFF as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0xDB as f32 / 255.0f32,
        0xE4 as f32 / 255.0f32,
        0xFF as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0xBA as f32 / 255.0f32,
        0xC8 as f32 / 255.0f32,
        0xFF as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0x91 as f32 / 255.0f32,
        0xA7 as f32 / 255.0f32,
        0xFF as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0x74 as f32 / 255.0f32,
        0x8F as f32 / 255.0f32,
        0xFC as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0x5C as f32 / 255.0f32,
        0x7C as f32 / 255.0f32,
        0xFA as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0x4C as f32 / 255.0f32,
        0x6E as f32 / 255.0f32,
        0xF5 as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0x42 as f32 / 255.0f32,
        0x63 as f32 / 255.0f32,
        0xEB as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0x3B as f32 / 255.0f32,
        0x5B as f32 / 255.0f32,
        0xDB as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0x36 as f32 / 255.0f32,
        0x4F as f32 / 255.0f32,
        0xC7 as f32 / 255.0f32,
    ),
];

#[allow(dead_code)]
pub const BLUE: [Color; 10] = [
    Color::from_rgb(
        0xE7 as f32 / 255.0f32,
        0xF5 as f32 / 255.0f32,
        0xFF as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0xD0 as f32 / 255.0f32,
        0xEB as f32 / 255.0f32,
        0xFF as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0xA5 as f32 / 255.0f32,
        0xD8 as f32 / 255.0f32,
        0xFF as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0x74 as f32 / 255.0f32,
        0xC0 as f32 / 255.0f32,
        0xFC as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0x4D as f32 / 255.0f32,
        0xAB as f32 / 255.0f32,
        0xF7 as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0x33 as f32 / 255.0f32,
        0x9A as f32 / 255.0f32,
        0xF0 as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0x22 as f32 / 255.0f32,
        0x8B as f32 / 255.0f32,
        0xE6 as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0x1C as f32 / 255.0f32,
        0x7E as f32 / 255.0f32,
        0xD6 as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0x19 as f32 / 255.0f32,
        0x71 as f32 / 255.0f32,
        0xC2 as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0x18 as f32 / 255.0f32,
        0x64 as f32 / 255.0f32,
        0xAB as f32 / 255.0f32,
    ),
];

#[allow(dead_code)]
pub const CYAN: [Color; 10] = [
    Color::from_rgb(
        0xE3 as f32 / 255.0f32,
        0xFA as f32 / 255.0f32,
        0xFC as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0xC5 as f32 / 255.0f32,
        0xF6 as f32 / 255.0f32,
        0xFA as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0x99 as f32 / 255.0f32,
        0xE9 as f32 / 255.0f32,
        0xF2 as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0x66 as f32 / 255.0f32,
        0xD9 as f32 / 255.0f32,
        0xE8 as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0x3B as f32 / 255.0f32,
        0xC9 as f32 / 255.0f32,
        0xDB as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0x22 as f32 / 255.0f32,
        0xB8 as f32 / 255.0f32,
        0xCF as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0x15 as f32 / 255.0f32,
        0xAA as f32 / 255.0f32,
        0xBF as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0x10 as f32 / 255.0f32,
        0x98 as f32 / 255.0f32,
        0xAD as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0x0C as f32 / 255.0f32,
        0x85 as f32 / 255.0f32,
        0x99 as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0x0B as f32 / 255.0f32,
        0x72 as f32 / 255.0f32,
        0x85 as f32 / 255.0f32,
    ),
];

#[allow(dead_code)]
pub const TEAL: [Color; 10] = [
    Color::from_rgb(
        0xE6 as f32 / 255.0f32,
        0xFC as f32 / 255.0f32,
        0xF5 as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0xC3 as f32 / 255.0f32,
        0xFA as f32 / 255.0f32,
        0xE8 as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0x96 as f32 / 255.0f32,
        0xF2 as f32 / 255.0f32,
        0xD7 as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0x63 as f32 / 255.0f32,
        0xE6 as f32 / 255.0f32,
        0xBE as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0x38 as f32 / 255.0f32,
        0xD9 as f32 / 255.0f32,
        0xA9 as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0x20 as f32 / 255.0f32,
        0xC9 as f32 / 255.0f32,
        0x97 as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0x12 as f32 / 255.0f32,
        0xB8 as f32 / 255.0f32,
        0x86 as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0x0C as f32 / 255.0f32,
        0xA6 as f32 / 255.0f32,
        0x78 as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0x09 as f32 / 255.0f32,
        0x92 as f32 / 255.0f32,
        0x68 as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0x08 as f32 / 255.0f32,
        0x7F as f32 / 255.0f32,
        0x5B as f32 / 255.0f32,
    ),
];

#[allow(dead_code)]
pub const GREEN: [Color; 10] = [
    Color::from_rgb(
        0xEB as f32 / 255.0f32,
        0xFB as f32 / 255.0f32,
        0xEE as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0xD3 as f32 / 255.0f32,
        0xF9 as f32 / 255.0f32,
        0xD8 as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0xB2 as f32 / 255.0f32,
        0xF2 as f32 / 255.0f32,
        0xBB as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0x8C as f32 / 255.0f32,
        0xE9 as f32 / 255.0f32,
        0x9A as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0x69 as f32 / 255.0f32,
        0xDB as f32 / 255.0f32,
        0x7C as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0x51 as f32 / 255.0f32,
        0xCF as f32 / 255.0f32,
        0x66 as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0x40 as f32 / 255.0f32,
        0xC0 as f32 / 255.0f32,
        0x57 as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0x37 as f32 / 255.0f32,
        0xB2 as f32 / 255.0f32,
        0x4D as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0x2F as f32 / 255.0f32,
        0x9E as f32 / 255.0f32,
        0x44 as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0x2B as f32 / 255.0f32,
        0x8A as f32 / 255.0f32,
        0x3E as f32 / 255.0f32,
    ),
];

#[allow(dead_code)]
pub const LIME: [Color; 10] = [
    Color::from_rgb(
        0xF4 as f32 / 255.0f32,
        0xFC as f32 / 255.0f32,
        0xE3 as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0xE9 as f32 / 255.0f32,
        0xFA as f32 / 255.0f32,
        0xC8 as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0xD8 as f32 / 255.0f32,
        0xF5 as f32 / 255.0f32,
        0xA2 as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0xC0 as f32 / 255.0f32,
        0xEB as f32 / 255.0f32,
        0x75 as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0xA9 as f32 / 255.0f32,
        0xE3 as f32 / 255.0f32,
        0x4B as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0x94 as f32 / 255.0f32,
        0xD8 as f32 / 255.0f32,
        0x2D as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0x82 as f32 / 255.0f32,
        0xC9 as f32 / 255.0f32,
        0x1E as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0x74 as f32 / 255.0f32,
        0xB8 as f32 / 255.0f32,
        0x16 as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0x66 as f32 / 255.0f32,
        0xA8 as f32 / 255.0f32,
        0x0F as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0x5C as f32 / 255.0f32,
        0x94 as f32 / 255.0f32,
        0x0D as f32 / 255.0f32,
    ),
];

#[allow(dead_code)]
pub const YELLOW: [Color; 10] = [
    Color::from_rgb(
        0xFF as f32 / 255.0f32,
        0xF9 as f32 / 255.0f32,
        0xDB as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0xFF as f32 / 255.0f32,
        0xF3 as f32 / 255.0f32,
        0xBF as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0xFF as f32 / 255.0f32,
        0xEC as f32 / 255.0f32,
        0x99 as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0xFF as f32 / 255.0f32,
        0xE0 as f32 / 255.0f32,
        0x66 as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0xFF as f32 / 255.0f32,
        0xD4 as f32 / 255.0f32,
        0x3B as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0xFC as f32 / 255.0f32,
        0xC4 as f32 / 255.0f32,
        0x19 as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0xFA as f32 / 255.0f32,
        0xB0 as f32 / 255.0f32,
        0x05 as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0xF5 as f32 / 255.0f32,
        0x9F as f32 / 255.0f32,
        0x00 as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0xF0 as f32 / 255.0f32,
        0x8C as f32 / 255.0f32,
        0x00 as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0xE6 as f32 / 255.0f32,
        0x77 as f32 / 255.0f32,
        0x00 as f32 / 255.0f32,
    ),
];

#[allow(dead_code)]
pub const ORANGE: [Color; 10] = [
    Color::from_rgb(
        0xFF as f32 / 255.0f32,
        0xF4 as f32 / 255.0f32,
        0xE6 as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0xFF as f32 / 255.0f32,
        0xE8 as f32 / 255.0f32,
        0xCC as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0xFF as f32 / 255.0f32,
        0xD8 as f32 / 255.0f32,
        0xA8 as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0xFF as f32 / 255.0f32,
        0xC0 as f32 / 255.0f32,
        0x78 as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0xFF as f32 / 255.0f32,
        0xA9 as f32 / 255.0f32,
        0x4D as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0xFF as f32 / 255.0f32,
        0x92 as f32 / 255.0f32,
        0x2B as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0xFD as f32 / 255.0f32,
        0x7E as f32 / 255.0f32,
        0x14 as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0xF7 as f32 / 255.0f32,
        0x67 as f32 / 255.0f32,
        0x07 as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0xE8 as f32 / 255.0f32,
        0x59 as f32 / 255.0f32,
        0x0C as f32 / 255.0f32,
    ),
    Color::from_rgb(
        0xD9 as f32 / 255.0f32,
        0x48 as f32 / 255.0f32,
        0x0F as f32 / 255.0f32,
    ),
];
