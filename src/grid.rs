use coord::Coord;

#[derive(Clone, Debug)]
pub struct Grid<T> {
    pub width: usize,
    pub data: Vec<T>,
}

impl<T> Grid<T> {
    pub fn new(width: usize, height: usize) -> Self {
        let data: Vec<T> = Vec::with_capacity(width * height);
        Grid { data, width }
    }

    // Use options for out of bound coords
    pub fn get(&self, coord: Coord) -> &T {
        let index = (coord.x + self.width as i32 * coord.y) as usize;
        &self.data[index]
    }

    // Use options for out of bounds coords
    pub fn set(&mut self, coord: Coord, value: T) {
        let index = (coord.x + self.width as i32 * coord.y) as usize;
        self.data[index] = value
    }

    // Height
    // pub fn height(&self) -> usize {
    //     self.data.len() / self.width
    // }
}
