extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct BoolField {
    pub width: usize,
    data: Vec<bool>,
}

impl BoolField {
    pub fn new(width: usize, height: usize) -> Self {
        let data: Vec<bool> = vec![false; width * height];
        BoolField { data, width }
    }

    pub fn get(&self, x: usize, y: usize) -> bool {
        self.data[x + self.width * y]
    }

    pub fn set(&mut self, x: usize, y: usize, value: bool) {
        self.data[x + self.width * y] = value
    }

    pub fn height(&self) -> usize {
        self.data.len() / self.width
    }
}
