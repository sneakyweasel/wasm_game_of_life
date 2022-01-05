extern crate colored;
use color::Color;
use colored::*;
use complex::Complex;
use coord::Coord;
use std::fmt;

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

/// Implement display for the cells
impl fmt::Display for Grid<bool> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.data.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            writeln!(f)?;
        }
        Ok(())
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

/// Implement display for the cells
impl fmt::Display for Grid<f32> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.data.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let rgb = (255. - cell * 255.0) as u8;
                if cell == 0.0 {
                    let symbol = "◼".black();
                    write!(f, "{}", symbol)?;
                } else {
                    let symbol = "◼".truecolor(rgb, rgb, rgb);
                    write!(f, "{}", symbol)?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
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

    #[allow(dead_code)]
    /// Convert quantum to rgb colors.
    pub fn into_cells(self) -> Grid<Color> {
        let mut colors: Grid<Color> = Grid::<Color>::new(self.width, self.height());
        for (i, c) in self.data.iter().enumerate() {
            colors.data[i] = c.into_rgb();
        }
        colors
    }
}

/// Implement display for the cells
impl fmt::Display for Grid<Complex> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.data.as_slice().chunks(self.width as usize) {
            for &c in line {
                let color = c.into_rgb();
                let symbol = "◼".truecolor(color.r, color.g, color.b);
                write!(f, "{}", symbol)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

/// Implement display for the cells
impl Grid<Color> {
    /// Create a new grid of the given width and height.
    pub fn new(width: usize, height: usize) -> Self {
        Grid {
            width,
            data: vec![Color::white(); width * height],
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

    /// Returns true if the coord is valid
    pub fn is_valid_coord(&self, coord: &Coord) -> bool {
        coord.x < self.width as i32 && coord.x >= 0 && coord.y < self.width as i32 && coord.y >= 0
    }

    /// Get neighbors of the given coord.
    pub fn valid_neighbors(&self, coord: Coord) -> Vec<Coord> {
        coord
            .neighbors()
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
