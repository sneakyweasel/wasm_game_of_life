extern crate wasm_bindgen;

use color::Color;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cell {
    pub color: Color,
}

impl Cell {
    pub fn toggle(&mut self) {
        self.color = self.color.complementary();
    }

    pub fn new(color: Color) -> Cell {
        Cell { color }
    }
}
