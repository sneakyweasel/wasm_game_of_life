extern crate cfg_if;
extern crate priority_queue;
extern crate rand;
extern crate wasm_bindgen;
extern crate web_sys;

mod bool_field;
mod cell;
mod color;
mod complex;
mod complex_field;
mod float_field;
mod quantum;
mod utils;

use bool_field::BoolField;
use cell::Cell;
use color::Color;
use complex::Complex;
use complex_field::ComplexField;
use float_field::FloatField;
use priority_queue::PriorityQueue;
use std::f32::consts::PI;
use std::fmt;
use wasm_bindgen::prelude::*;
use web_sys::console;

#[derive(PartialEq, Eq, Hash)]
pub struct Pixel {
    x: i32,
    y: i32,
    d: i32,
}

impl Pixel {
    pub fn new(x: i32, y: i32, d: i32) -> Self {
        Pixel { x, y, d }
    }
}

pub struct Timer<'a> {
    name: &'a str,
}

impl<'a> Timer<'a> {
    pub fn new(name: &'a str) -> Timer<'a> {
        console::time_with_label(name);

        Timer { name }
    }
}

impl<'a> Drop for Timer<'a> {
    fn drop(&mut self) {
        console::time_end_with_label(self.name);
    }
}

#[wasm_bindgen]
pub struct Universe {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
    complex_field: ComplexField,
    walls: BoolField,
    sinks: BoolField,
    sink_mult: FloatField,
    level_design_potential: FloatField,
    potential_cache: FloatField,
    max_tilt: f32,
    dt: f32,
}

impl Universe {
    fn get_index(&self, row: usize, column: usize) -> usize {
        (row * self.width + column) as usize
    }

    /// Get the dead and alive values of the entire universe.
    pub fn get_cells(&self) -> &[Cell] {
        &self.cells
    }

    /// Set cells to be alive in a universe by passing the row and column
    /// of each cell as an array.
    pub fn set_cells(&mut self, cells: &[(usize, usize)]) {
        for (row, column) in cells.iter().cloned() {
            let idx = self.get_index(row, column);
            self.cells[idx] = Cell {
                color: Color::new(0, 0, 0),
            };
        }
    }
}

/// Public methods, exported to JavaScript.
#[wasm_bindgen]
impl Universe {
    pub fn new() -> Universe {
        utils::set_panic_hook();

        let height = 50;
        let width = 50;
        let cells = (0..width * height)
            .map(|_i| Cell::new(Color::WHITE()))
            .collect();
        let complex_field = ComplexField::new(width, height);
        let walls = BoolField::new(width, height);
        let sinks = BoolField::new(width, height);
        let sink_mult = FloatField::new(width, height);
        let level_design_potential = FloatField::new(width, height);
        let potential_cache = FloatField::new(width, height);

        Universe {
            width: width,
            height: height,
            cells,
            complex_field,
            walls,
            sinks,
            sink_mult,
            level_design_potential,
            potential_cache,
            max_tilt: 0.0,
            dt: 0.0,
        }
    }

    /// Process the next generation of the universe.
    pub fn tick(&mut self) {
        // let _timer = Timer::new("Universe::tick");
        let mut next = self.cells.clone();
        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                // let cell = self.cells[idx];
                let next_cell = Cell::new(Color::random());
                next[idx] = next_cell;
            }
        }
        self.cells = next;
    }

    /// Get the width of the universe.
    pub fn width(&self) -> usize {
        self.width
    }

    /// Get the height of the universe.
    pub fn height(&self) -> usize {
        self.height
    }

    /// Set the width of the universe.
    /// Resets all cells to the blank state.
    pub fn set_width(&mut self, width: usize) {
        self.width = width;
        self.cells = (0..width * self.height)
            .map(|_i| Cell::new(Color::WHITE()))
            .collect();
    }

    /// Set the height of the universe.
    /// Resets all cells to the blank state.
    pub fn set_height(&mut self, height: usize) {
        self.height = height;
        self.cells = (0..self.width * height)
            .map(|_i| Cell::new(Color::WHITE()))
            .collect();
    }

    /// Get the cells of the universe
    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    /// toggle the state of the specified cell
    pub fn toggle_cell(&mut self, row: usize, column: usize) {
        let idx = self.get_index(row, column);
        self.cells[idx].toggle();
    }

    /// Set max tilt
    pub fn set_max_tilt(mut self, mt: f32) {
        self.max_tilt = mt;
    }

    /// Set dt
    pub fn set_time_delta(mut self, dt: f32) {
        self.dt = dt;
    }

    /// Set sinks
    fn set_sink(&mut self, sub_mask: BoolField) {
        self.sinks = sub_mask
    }

    /// Set walls
    fn set_walls(&mut self, sub_mask: BoolField) {
        self.walls = sub_mask
    }

    /// Set the complex field to zero if there is a wall at the specified cell
    fn add_walls(&mut self) {
        for x in 1..self.width {
            for y in 1..self.height {
                if self.walls.get(x, y) {
                    self.complex_field.set(x, y, Complex::ZERO());
                }
            }
        }
    }

    /// Flood fill sink_mult with 0 where not a sink; otherwise distance in pixels from non-sink
    /// ...basically a mini-Dijkstra
    fn setup_sink_mult(&mut self) {
        let mut queue: PriorityQueue<Pixel, i32> = PriorityQueue::new();
        for y in 0..self.height {
            for x in 0..self.width {
                let priority = (x * self.width + y) as i32;
                self.sink_mult.set(x, y, f32::INFINITY);
                if !self.sinks.get(x, y) && !self.walls.get(x, y) {
                    queue.push(Pixel::new(x as i32, y as i32, 0), priority);
                }
            }
            while !queue.is_empty() {
                let p: Pixel = queue.pop().unwrap().0;
                if self.sink_mult.get(p.x as usize, p.y as usize) > p.d as f32 {
                    self.sink_mult.set(p.x as usize, p.y as usize, p.d as f32);

                    for dx in (-1i32..1).step_by(2) {
                        for dy in (-1i32..1).step_by(2) {
                            let q: Pixel = Pixel::new(p.x + dx, p.y + dy, p.d + 1);
                            if q.x >= 0
                                && q.x < self.width as i32
                                && q.y >= 0
                                && q.y < self.height as i32
                            {
                                queue.push(q, 1);
                            }
                        }
                    }
                }
            }
        }
        //now convert these to actual sink_mults
        let suddenness: f32 = 0.005;
        for y in 0..self.height {
            for x in 0..self.width {
                let dist = self.sink_mult.get(x, y);
                let value = (-(dist / 2.0).powf(2.0) * suddenness).exp();
                self.sink_mult.set(x, y, value);
            }
        }
    }

    // Add a gaussian distribution to the complex field
    pub fn add_gaussian(&mut self, xc: i32, yc: i32, sigma: f32, fx: f32, fy: f32, a_scale: f32) {
        let a: f32 = a_scale * 2.0 * PI;
        let d: f32 = 4.0 * sigma * sigma;
        let omega_x = 2.0 * PI * fx; // seems wrong
        let omega_y = 2.0 * PI * fy; // seems wrong

        for x in 1..&self.width - 1 {
            for y in 1..&self.height - 1 {
                let r2 =
                    ((x as i32) - xc) * ((x as i32) - xc) + ((y as i32) - yc) * ((y as i32) - yc);
                let re = a
                    * f32::exp(-(r2 as f32) / d)
                    * (omega_x * (x as f32) / (self.width as f32)).cos()
                    * (omega_y * (y as f32) / (self.height as f32)).cos();
                let im = a
                    * f32::exp(-(r2 as f32) / d)
                    * (omega_x * (x as f32) / (self.width as f32)).sin()
                    * (omega_y * (y as f32) / (self.height as f32)).sin();

                let c = Complex::new(re, im);
                let orig_c = *self.complex_field.get(x, y);
                self.complex_field.set(x, y, orig_c.add(&c));
            }
        }
    }

    /// Get potential at a given point
    fn get_potential(&self, x: usize, y: usize) -> f32 {
        self.level_design_potential.get(x, y)
    }

    /// Add potential to a given point
    fn add_potential(&mut self, x: usize, y: usize, pot: f32) {
        let orig = self.get_potential(x, y);
        self.level_design_potential.set(x, y, orig + pot)
    }

    /// Add potential cone starting from a given point
    pub fn add_potential_cone(&mut self, xc: i32, yc: i32, radius: f32, depth: f32) {
        for y in 0..self.height {
            for x in 0..self.width {
                let dx = x as i32 - xc;
                let dy = y as i32 - yc;
                let r = ((dx * dx + dy * dy) as f32).sqrt();
                if r < radius {
                    let pot = r / radius * depth;
                    self.add_potential(x, y, pot);
                }
            }
        }
    }

    /// potentials > 0 are problematic
    /// pixel wide band with potential +1 above background - tunnelling
    /// potential of -5 over width of universe - good for steering
    fn reset_potential_cache(&mut self, x_slope: f32, y_slope: f32) {
        //if tilting 2 directions at once reduce tilt to compensate
        let total_slope = x_slope.abs() + y_slope.abs();
        let tilt = if total_slope <= 1.0 {
            self.max_tilt
        } else {
            self.max_tilt / total_slope
        };

        //compute desired relative potentials of corners
        let largest_dim = usize::max(self.width, self.height);
        let right_change = -x_slope * tilt * (self.width / largest_dim) as f32;
        let down_change = -y_slope * tilt * (self.height / largest_dim) as f32;
        let top_left = -right_change - down_change;
        let top_right = right_change - down_change;
        let down_left = -right_change + down_change;
        let down_right = right_change + down_change;

        //adjust all potentials to be < 0
        let max = [top_left, top_right, down_left, down_right]
            .iter()
            .copied()
            .fold(f32::NEG_INFINITY, f32::max);
        let new_top_left = top_left - max;

        //compute per-simulation-element steps in potential to efficiently compute
        let x_pot_step = right_change / self.width as f32;
        let y_pot_step = down_change / self.height as f32;
        let mut left_edge_pot = new_top_left;
        for y in 1..self.height - 1 {
            left_edge_pot += y_pot_step;
            let mut current_pot = left_edge_pot;
            for x in 1..self.width - 1 {
                current_pot += x_pot_step;
                self.potential_cache
                    .set(x, y, current_pot + self.level_design_potential.get(x, y))
            }
        }
    }

    /// Ensure there is no positive potential
    /// Get max potential and subtract from all cells
    fn ensure_no_positive_potential(&mut self) {
        let mut max_pot = f32::NEG_INFINITY;
        for y in 0..self.height {
            for x in 0..self.width {
                let pot = self.level_design_potential.get(x, y);
                if pot > max_pot {
                    max_pot = pot;
                }
            }
        }
        for y in 0..self.height {
            for x in 0..self.width {
                let value = self.level_design_potential.get(x, y) - max_pot;
                self.level_design_potential.set(x, y, value);
            }
        }
    }

    fn clear_complex_field(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                self.complex_field.set(x, y, Complex::ZERO());
            }
        }
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell.color.is_white() { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}
