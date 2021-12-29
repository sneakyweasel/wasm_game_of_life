extern crate cfg_if;
extern crate priority_queue;
extern crate rand;
extern crate wasm_bindgen;
extern crate web_sys;

mod pixel;
mod cell;
mod color;
mod complex;
mod coord;
mod grid;
mod utils;

use pixel::Pixel;
use cell::Cell;
use complex::Complex;
use coord::Coord;
use grid::Grid;
use priority_queue::PriorityQueue;
use std::f32::consts::PI;
use std::fmt;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Universe {
    width: usize,
    height: usize,
    cells: Grid<Cell>,
    quantum: Grid<Complex>,
    walls: Grid<bool>,
    sinks: Grid<bool>,
    sink_mult: Grid<f32>,
    potential_level: Grid<f32>,
    potential_cache: Grid<f32>,
    max_tilt: f32,
    dt: f32,
}

/// Public methods, exported to JavaScript.
#[wasm_bindgen]
impl Universe {
    pub fn new() -> Universe {
        utils::set_panic_hook();
        let height = 50;
        let width = 50;
        let max_tilt = 0.0;
        let dt = 0.1;

        // Create a new grid of the given size
        let cells = Grid::<Cell>::new(width, height);
        let quantum = Grid::<Complex>::new(width, height);
        let walls  = Grid::<bool>::new(width, height);
        let sinks = Grid::<bool>::new(width, height);
        let sink_mult = Grid::<f32>::new(width, height);
        let potential_level = Grid::<f32>::new(width, height);
        let potential_cache = Grid::<f32>::new(width, height);
        
        Universe {
            width,
            height,
            cells,
            quantum,
            walls,
            sinks,
            sink_mult,
            potential_level,
            potential_cache,
            max_tilt,
            dt,
        }
    }

    /// Initialize universe with a default level.
    pub fn setup(&mut self) {
        self.dt = 0.1;
        self.max_tilt = 2.5;
        let _scale = 3.0;
        let _qft = 5;
        self.add_gaussian(Coord::new(3, 3), 1.0, 0.0, 0.0, 1.0);
        self.setup_sink_mult();
        self.setup_walls();
        self.ensure_no_positive_potential();
        self.cells = self.quantum.clone().into_cells();
    }

    /// Process the next generation of the universe.
    pub fn tick(&mut self) {
        let next = self.cells.clone();
        self.step();
        self.cells = next;
    }

    /// Compute the steps throught the quantum field theory.
    pub fn step(&mut self) {
        let dt = self.dt;
        let x_slope = 0.0;
        let y_slope = 0.0;
        self.reset_potential_cache(x_slope, y_slope);

        for y in 1..self.height {
            for x in 1..self.width {
                let coord = Coord::new(x as i32, y as i32);
                
                if !self.is_wall(coord) {
                    let sink_mult = self.sink_mult.get(coord).unwrap();
                    let potential_cache = self.potential_cache.get(coord).unwrap();
                    
                    let cx = self.quantum.get(coord).unwrap();
                    let top = self.quantum.get(coord.top()).unwrap();
                    let bottom = self.quantum.get(coord.bottom()).unwrap();
                    let left = self.quantum.get(coord.left()).unwrap();
                    let right = self.quantum.get(coord.right()).unwrap();

                    let re = sink_mult
                        * (cx.im
                            + dt * (-0.5 * (top.re + bottom.re + left.re + right.re - 4.0 * cx.re)
                                + potential_cache * cx.re));
                    let im = sink_mult
                        * (cx.re
                            + dt * (-0.5 * (top.im + bottom.im + left.im + right.im - 4.0 * cx.im)
                                + potential_cache * cx.im));

                    self.quantum.set(coord, Complex::new(re, im));                    
                }
            }
        }
    }

    /// Get the cells of the universe
    pub fn cells(&self) -> *const Cell {
        self.cells.data.as_ptr()
    }

    /// toggle the state of the specified cell
    pub fn toggle_cell(&self, coord: Coord) {
        let mut cell: Cell = *self.cells.get(coord).unwrap();
        cell.toggle();
    }

    /// Check if there is a wall at coord
    pub fn is_wall(&self, coord: Coord) -> bool {
        *self.walls.get(coord).unwrap()
    }

    /// Check if there is a wall at coord
    pub fn is_sink(&self, coord: Coord) -> bool {
        *self.sinks.get(coord).unwrap()
    }

    /// Set the complex field to zero if there is a wall at the specified cell
    fn setup_walls(&mut self) {
        for i in 0..self.walls.size() {
            if self.walls.data[i] {
                self.quantum.data[i] = Complex::zero();
            }
        }
    }

    /// Flood fill sink_mult with 0 where not a sink; otherwise distance in pixels from non-sink
    /// ...basically a mini-Dijkstra
    fn setup_sink_mult(&mut self) {
        let mut queue: PriorityQueue<Pixel, i32> = PriorityQueue::new();
        for y in 0..self.height {
            for x in 0..self.width {
                let coord = Coord::new(x as i32, y as i32);
                // Fill priority queue
                let priority = (x * self.width + y) as i32;
                self.sink_mult.set(coord, f32::INFINITY);
                if !self.is_wall(coord) && !self.is_sink(coord) {
                    queue.push(Pixel::new(coord, 0), priority);
                }
            }
            while !queue.is_empty() {
                let p: Pixel = queue.pop().unwrap().0;
                if *self.sink_mult.get(p.coord).unwrap() > p.val as f32 {
                    self.sink_mult.set(p.coord, p.val as f32);
                    
                    let neighbors = self.sink_mult.valid_neighbors(p.coord);
                    for coord in neighbors {
                        let q: Pixel = Pixel::new(coord, p.val + 1);
                        queue.push(q, 1);
                    }
                }
            }
        }
        //now convert these to actual sink_mults
        let suddenness: f32 = 0.005;
        for y in 0..=self.height {
            for x in 0..=self.width {
                let coord = Coord::new(x as i32, y as i32);

                let dist = self.sink_mult.get(coord).unwrap();
                let value = (-(dist / 2.0).powf(2.0) * suddenness).exp();
                self.sink_mult.set(coord, value);
            }
        }
    }

    /// Add a gaussian distribution to the complex field
    pub fn add_gaussian(&mut self, c: Coord, sigma: f32, fx: f32, fy: f32, a_scale: f32) {
        let a: f32 = a_scale * 2.0 * PI;
        let d: f32 = 4.0 * sigma * sigma;
        let omega_x = 2.0 * PI * fx; // seems wrong
        let omega_y = 2.0 * PI * fy; // seems wrong

        // TODO: use neighbourgs to speed this up
        let fwidth = self.width as f32;
        let fheight = self.height as f32;
        for x in 1..self.width {
            for y in 1..self.height {
                let fx = x as f32;
                let fy = y as f32;
                
                let coord = Coord::new(x as i32, y as i32);
                let r2: f32 = ((coord.x - c.x).pow(2) + (coord.y - c.y).pow(2)) as f32;
                let re = a
                    * f32::exp(-r2 / d)
                    * (omega_x * fx / fwidth).cos()
                    * (omega_y * fy / fheight).cos();
                let im = a
                    * f32::exp(-r2 / d)
                    * (omega_x * fx / fwidth).sin()
                    * (omega_y * fy / fheight).sin();

                let c = Complex::new(re, im);
                self.quantum.add(coord, c);
            }
        }
    }

    /// Add potential cone starting from a given point
    pub fn add_potential_cone(&mut self, xc: i32, yc: i32, radius: f32, depth: f32) {
        for y in 0..=self.height {
            for x in 0..=self.width {
                let coord = Coord::new(x as i32, y as i32);
                let dx = x as i32 - xc;
                let dy = y as i32 - yc;
                let r = ((dx * dx + dy * dy) as f32).sqrt();
                if r < radius {
                    let pot = r / radius * depth;
                    self.potential_level.add(coord, pot);
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
        for y in 1..self.height {
            left_edge_pot += y_pot_step;
            let mut current_pot = left_edge_pot;
            for x in 1..self.width {
                let coord = Coord::new(x as i32, y as i32);
                current_pot += x_pot_step;
                self.potential_cache
                    .set(coord, current_pot + self.potential_level.get(coord).unwrap());
            }
        }
    }

    /// Ensure there is no positive potential
    /// Get max potential and subtract from all cells
    fn ensure_no_positive_potential(&mut self) {
        let max_pot = self.potential_level.data.iter().cloned().fold(1./0., f32::max);
        
        for y in 0..=self.height {
            for x in 0..=self.width {
                let coord = Coord::new(x as i32, y as i32);
                self.potential_level.add(coord, -max_pot);
            }
        }
    }
}

/// Implement display for the cells
impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.data.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell.color.is_white() { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_universe_creation() {
    let u = Universe::new();
    assert_eq!(u.width, 50);
    assert_eq!(u.height, 50);
    assert_eq!(u.cells.data.len(), 2500);
}
