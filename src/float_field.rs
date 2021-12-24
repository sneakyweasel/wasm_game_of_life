extern crate wasm_bindgen;

use std::fmt;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct FloatField {
    pub width: usize,
    data: Vec<f32>,
}

impl FloatField {
    pub fn new(width: usize, height: usize) -> Self {
        let data: Vec<f32> = vec![0.0; width * height];
        FloatField { data, width }
    }

    pub fn set_equal_to(&self, other: FloatField) -> Self {
        FloatField {
            width: other.width,
            data: other.data.clone(),
        }
    }

    pub fn get(&self, x: usize, y: usize) -> f32 {
        self.data[x + self.width * y]
    }

    pub fn set(&mut self, x: usize, y: usize, value: f32) {
        self.data[x + self.width * y] = value
    }

    pub fn del2(&self, x: usize, y: usize) -> f32 {
        self.get(x, y - 1) + self.get(x, y + 1) + self.get(x - 1, y) + self.get(x + 1, y)
            - 4.0 * self.get(x, y)
    }
}

impl fmt::Display for FloatField {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.data)
    }
}
