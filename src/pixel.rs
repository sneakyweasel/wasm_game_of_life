use coord::Coord;

#[derive(PartialEq, Eq, Hash)]
pub struct Pixel {
    pub coord: Coord,
    pub val: i32,
}

impl Pixel {
    pub fn new(coord: Coord, val: i32) -> Self {
        Pixel { coord, val }
    }
}