use coord::Coord;
use color::Color;
use cell::Cell;
use complex::Complex;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Grid<T> {
    pub width: usize,
    pub data: Vec<T>,
}

/// Specific methods for boolean grids.
impl Grid<bool> {
    /// Create a new grid of the given width and height.
    pub fn new(width: usize, height: usize) -> Self {
        Grid {
            width,
            data: vec![false; width * height],
        }
    }
}

/// Specific methods for f32 grids.
impl Grid<f32> {
    /// Create a new grid of the given width and height.
    pub fn new(width: usize, height: usize) -> Self {
        Grid {
            width,
            data: vec![0.0; width * height],
        }
    }

    /// Add a value to the original value at the given coord.
    pub fn add(&mut self, coord: Coord, value: f32) {
        if self.is_valid_coord(&coord) {
            let orig = *self.get(coord).unwrap();
            self.set(coord, orig + value);
        } else {
            panic!("Invalid coord: {:?}", coord);
        }
    }
}

/// Specific methods for complex grids.
impl Grid<Complex> {
    /// Create a new grid of the given width and height.
    pub fn new(width: usize, height: usize) -> Self {
        Grid {
            width,
            data: vec![Complex::zero(); width * height],
        }
    }

    /// Add a value to the original value at the given coord.
    pub fn add(&mut self, coord: Coord, value: Complex) {
        if self.is_valid_coord(&coord) {
            let orig = *self.get(coord).unwrap();
            self.set(coord, orig.add(&value));
        } else {
            panic!("Invalid coord: {:?}", coord);
        }
    }

    pub fn into_cells(self) -> Grid<Cell> {
        let mut cells: Grid<Cell> = Grid::<Cell>::new(self.width, self.height());
        for (i, c) in self.data.iter().enumerate() {
            cells.data[i] = Cell::new(c.into_hsl());
        }
        cells
    }
}

/// Specific methods for cell grids.
impl Grid<Cell> {
    /// Create a new grid of the given width and height.
    pub fn new(width: usize, height: usize) -> Self {
        Grid {
            width,
            data: vec![Cell::new(Color::white()); width * height],
        }
    }
}

/// Generic grid methods.
impl<T> Grid<T> {
    /// Returns the value at the given coord.
    pub fn get(&self, coord: Coord) -> Option<&T> {
        if self.is_valid_coord(&coord) {
            Some(&self.data[coord.x as usize + coord.y as usize * self.width])
        } else {
            None
        }
    }

    /// Sets the value at the given coord.
    pub fn set(&mut self, coord: Coord, value: T) {
        if self.is_valid_coord(&coord) {
            self.data[coord.x as usize + coord.y as usize * self.width] = value;
        } else {
            panic!("Invalid coord: {:?}", coord);
        }
    }

    /// Grid height.
    pub fn height(&self) -> usize {
        self.data.len() / self.width
    }

    /// Grid size.
    pub fn size(&self) -> usize {
        self.width * self.height()
    }

    /// Returns true if the coord is valid
    pub fn is_valid_coord(&self, coord: &Coord) -> bool {
        coord.x < self.width as i32 &&
        coord.x >= 0 &&
        coord.y < self.width as i32 &&
        coord.y >= 0
    }

    /// Get neighbors of the given coord.
    pub fn valid_neighbors(&self, coord: Coord) -> Vec<Coord> {
        coord.neighbors()
            .iter()
            .cloned()
            .filter(|c| self.is_valid_coord(c))
            .collect()
    }
}

#[cfg(test)]
#[test]
fn is_valid_coord_for_a_grid() {
    let grid: Grid<f32> = Grid::<f32>::new(5, 5);
    let valid = Coord::new(3, 2);
    let invalid = Coord::new(5, 8);
    assert!(grid.is_valid_coord(&valid)); 
    assert!(!grid.is_valid_coord(&invalid)); // out of bounds  
}

#[test]
fn returns_an_option_for_f32_grid_get_coord() {
    let grid: Grid<f32> = Grid::<f32>::new(5, 5);
    let valid = Coord::new(3, 2);
    let invalid = Coord::new(5, 8);
    assert_eq!(grid.get(valid), Some(&0.0)); 
    assert_eq!(grid.get(invalid), None); // out of bounds  
}
