extern crate cfg_if;
extern crate priority_queue;
extern crate rand;
extern crate wasm_bindgen;
extern crate web_sys;

mod cell;
mod color;
mod complex;
mod coord;
mod grid;
mod quantum;
mod timer;
mod utils;

use cell::Cell;
use color::Color;
use complex::Complex;
use coord::Coord;
use grid::Grid;
use priority_queue::PriorityQueue;
use std::f32::consts::PI;
use std::fmt;
use wasm_bindgen::prelude::*;

#[derive(PartialEq, Eq, Hash)]
pub struct Pixel {
    coord: Coord,
    val: i32,
}

impl Pixel {
    pub fn new(coord: Coord, val: i32) -> Self {
        Pixel { coord, val }
    }
}

#[wasm_bindgen]
pub struct Universe {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
    complex_field: Grid<Complex>,
    walls: Grid<bool>,
    sinks: Grid<bool>,
    sink_mult: Grid<f32>,
    level_design_potential: Grid<f32>,
    potential_cache: Grid<f32>,
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
            .map(|_i| Cell::new(Color::white()))
            .collect();
        let complex_field = Grid::new(width, height);
        let walls = Grid::new(width, height);
        let sinks = Grid::new(width, height);
        let sink_mult = Grid::new(width, height);
        let level_design_potential = Grid::new(width, height);
        let potential_cache = Grid::new(width, height);

        Universe {
            width,
            height,
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

    /// Initialize universe.
    pub fn setup(&mut self) {
        self.dt = 0.1;
        self.max_tilt = 2.5;
        let _scale = 3.0;
        let _qft = 5;
        self.add_gaussian(Coord::new(25, 25), 1.0, 0.0, 0.0, 1.0);
        self.setup_sink_mult();
        self.add_walls();
        self.ensure_no_positive_potential();
    }

    /// Process the next generation of the universe.
    pub fn tick(&mut self) {
        // let _timer = Timer::new("Universe::tick");
        let next = self.cells.clone();
        self.step();
        self.cells = next;
    }

    pub fn step(&mut self) {
        let x_slope = 0.0;
        let y_slope = 0.0;
        self.reset_potential_cache(x_slope, y_slope);

        for y in 1..self.height - 1 {
            for x in 1..self.width - 1 {
                let coord = Coord::new(x as i32, y as i32);
                let _sink_mult = self.sink_mult.get(coord);
                let _potential_cache = self.potential_cache.get(coord);
                // TODO
                // pub fn process(&mut self, x: usize, y: usize, sink_mult: f32, dt: f32, potential_cache: f32) {
                //   let cx = self.get(x, y);
                //   let top = self.get(x, y - 1);
                //   let bottom = self.get(x, y + 1);
                //   let left = self.get(x - 1, y);
                //   let right = self.get(x + 1, y);

                //   let re = sink_mult
                //       * (cx.im
                //           + dt * (-0.5 * (top.re + bottom.re + left.re + right.re - 4.0 * cx.re)
                //               + potential_cache * cx.re));
                //   let im = sink_mult
                //       * (cx.re
                //           + dt * (-0.5 * (top.im + bottom.im + left.im + right.im - 4.0 * cx.im)
                //               + potential_cache * cx.im));

                //   self.set(x, y, Complex::new(re, im));

                // if !self.walls.get(coord) {
                //     self.complex_field
                //         .process(x, y, sink_mult, self.dt, potential_cache);
                // }
            }
        }
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
            .map(|_i| Cell::new(Color::white()))
            .collect();
    }

    /// Set the height of the universe.
    /// Resets all cells to the blank state.
    pub fn set_height(&mut self, height: usize) {
        self.height = height;
        self.cells = (0..self.width * height)
            .map(|_i| Cell::new(Color::white()))
            .collect();
    }

    /// Get the cells of the universe
    // pub fn cells(&self) -> *const Cell {
    //     self.cells.as_ptr()
    // }

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

    /// Set the complex field to zero if there is a wall at the specified cell
    fn add_walls(&mut self) {
        for x in 1..self.width {
            for y in 1..self.height {
                let coord = Coord::new(x as i32, y as i32);
                if *self.walls.get(coord) {
                    self.complex_field.set(coord, Complex::zero());
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
                let coord = Coord::new(x as i32, y as i32);
                let priority = (x * self.width + y) as i32;
                self.sink_mult.set(coord, f32::INFINITY);
                if !*self.sinks.get(coord) && !*self.walls.get(coord) {
                    queue.push(Pixel::new(coord, 0), priority);
                }
            }
            while !queue.is_empty() {
                let p: Pixel = queue.pop().unwrap().0;
                if *self.sink_mult.get(p.coord) > p.val as f32 {
                    self.sink_mult.set(p.coord, p.val as f32);

                    for dx in (-1..1).step_by(2) {
                        for dy in (-1..1).step_by(2) {
                            let q_coord = Coord::new(dx, dy).add(p.coord);
                            let q: Pixel = Pixel::new(q_coord, p.val + 1);
                            if q.coord.x >= 0
                                && q.coord.x < self.width as i32
                                && q.coord.y >= 0
                                && q.coord.y < self.height as i32
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
                let coord = Coord::new(x as i32, y as i32);
                let dist = self.sink_mult.get(coord);
                let value = (-(dist / 2.0).powf(2.0) * suddenness).exp();
                self.sink_mult.set(coord, value);
            }
        }
    }

    // Add a gaussian distribution to the complex field
    pub fn add_gaussian(&mut self, c: Coord, sigma: f32, fx: f32, fy: f32, a_scale: f32) {
        let a: f32 = a_scale * 2.0 * PI;
        let d: f32 = 4.0 * sigma * sigma;
        let omega_x = 2.0 * PI * fx; // seems wrong
        let omega_y = 2.0 * PI * fy; // seems wrong

        // TODO: use neighbourgs to speed this up
        for x in 1..&self.width - 1 {
            for y in 1..&self.height - 1 {
                let coord = Coord::new(x as i32, y as i32);
                let r2: f32 = ((coord.x - c.x).pow(2) + (coord.y - c.y).pow(2)) as f32;
                let re = a
                    * f32::exp(-r2 / d)
                    * (omega_x * (x as f32) / (self.width as f32)).cos()
                    * (omega_y * (y as f32) / (self.height as f32)).cos();
                let im = a
                    * f32::exp(-r2 / d)
                    * (omega_x * (x as f32) / (self.width as f32)).sin()
                    * (omega_y * (y as f32) / (self.height as f32)).sin();

                let c = Complex::new(re, im);
                let orig_c = *self.complex_field.get(coord);
                self.complex_field.set(coord, orig_c.add(&c));
            }
        }
    }

    // /// Get potential at a given point
    // fn get_potential(&self, coord: Coord) -> f32 {
    //     *self.level_design_potential.get(coord)
    // }

    /// Add potential to a given point
    // fn add_potential(&mut self, coord: Coord, pot: f32) {
    //     *self.level_design_potential.get(coord)
    //     let orig = self.get_potential(coord);
    //     self.level_design_potential.set(coord, orig + pot)
    // }

    /// Add potential cone starting from a given point
    pub fn add_potential_cone(&mut self, xc: i32, yc: i32, radius: f32, depth: f32) {
        for y in 0..self.height {
            for x in 0..self.width {
                let coord = Coord::new(x as i32, y as i32);
                let dx = x as i32 - xc;
                let dy = y as i32 - yc;
                let r = ((dx * dx + dy * dy) as f32).sqrt();
                if r < radius {
                    let pot = r / radius * depth;
                    let orig = *self.level_design_potential.get(coord);
                    self.level_design_potential.set(coord, orig + pot);
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
                let coord = Coord::new(x as i32, y as i32);
                current_pot += x_pot_step;
                self.potential_cache
                    .set(coord, current_pot + self.level_design_potential.get(coord))
            }
        }
    }

    /// Ensure there is no positive potential
    /// Get max potential and subtract from all cells
    fn ensure_no_positive_potential(&mut self) {
        let mut max_pot = f32::NEG_INFINITY;
        for y in 0..self.height {
            for x in 0..self.width {
                let coord = Coord::new(x as i32, y as i32);
                let pot = *self.level_design_potential.get(coord);
                if pot > max_pot {
                    max_pot = pot;
                }
            }
        }
        for y in 0..self.height {
            for x in 0..self.width {
                let coord = Coord::new(x as i32, y as i32);
                let value = self.level_design_potential.get(coord) - max_pot;
                self.level_design_potential.set(coord, value);
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
