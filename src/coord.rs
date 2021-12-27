extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[wasm_bindgen]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}

impl Coord {
    pub fn new(x: i32, y: i32) -> Self {
        Coord { x, y }
    }

    pub fn add(&mut self, other: Coord) -> Self {
        Coord {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    pub fn index(&self, width: usize) -> i32 {
        self.y * width as i32 + self.x
    }

    pub fn top(&self) -> Coord {
        Coord::new(self.x, self.y - 1)
    }

    pub fn bottom(&self) -> Coord {
        Coord::new(self.x, self.y + 1)
    }

    pub fn left(&self) -> Coord {
        Coord::new(self.x - 1, self.y)
    }

    pub fn right(&self) -> Coord {
        Coord::new(self.x + 1, self.y)
    }

    pub fn neighbors(&self) -> [Coord; 4] {
        [self.top(), self.bottom(), self.left(), self.right()]
    }
}

#[cfg(test)]
#[test]
fn compute_index() {
    let coord = Coord::new(5, 8);
    let index = coord.index(5);
    assert_eq!(index, 45)
}

#[test]
fn add_coords() {
    let mut a = Coord::new(5, 8);
    let b = Coord::new(2, 1);
    assert_eq!(a.add(b), Coord::new(7, 9));
}
