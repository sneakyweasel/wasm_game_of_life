extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    pub fn WHITE() -> Color {
        Color {
            r: 255,
            g: 255,
            b: 255,
        }
    }

    pub fn new(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b }
    }

    pub fn random() -> Color {
        Color::new(
            rand::random::<u8>(),
            rand::random::<u8>(),
            rand::random::<u8>(),
        )
    }

    pub fn is_white(&self) -> bool {
        self.r == 255 && self.g == 255 && self.b == 255
    }

    pub fn complementary(&self) -> Color {
        Color::new(255 - self.r, 255 - self.g, 255 - self.b)
    }
}
