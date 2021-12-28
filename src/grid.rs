use coord::Coord;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Grid<T> {
    pub width: usize,
    pub data: Vec<T>,
}

impl<T> Grid<T> {
    /// Create a new grid of the given size.
    pub fn new(width: usize, height: usize) -> Self {
        let data: Vec<T> = Vec::with_capacity(width * height);
        Grid { data, width }
    }

    /// Returns the value at the given coord.
    pub fn get(&self, coord: Coord) -> Option<&T> {
        if self.is_valid_coord(coord) {
            Some(&self.data[coord.x as usize + coord.y as usize * self.width])
        } else {
            None
        }
    }

    /// Sets the value at the given coord.
    pub fn set(&mut self, coord: Coord, value: T) {
        if self.is_valid_coord(coord) {
            self.data[coord.x as usize + coord.y as usize * self.width] = value;
        } else {
            panic!("Invalid coord: {:?}", coord);
        }
    }

    /// Returns true if the coord is valid
    pub fn is_valid_coord(&self, coord: Coord) -> bool {
        coord.x < self.width as i32 &&
        coord.x >= 0 &&
        coord.y < self.width as i32 &&
        coord.y >= 0
    }
}
