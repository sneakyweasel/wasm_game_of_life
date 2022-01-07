extern crate cfg_if;
extern crate colored;
extern crate colorsys;
extern crate crossbeam_queue;
extern crate priority_queue;
extern crate rand;
extern crate wasm_bindgen;
extern crate web_sys;

mod cell;
mod color;
mod complex;
mod coord;
mod grid;
mod pixel;
mod utils;

use complex::Complex;
use coord::Coord;
use crossbeam_queue::SegQueue;
use grid::Grid;
use pixel::Pixel;
use std::f32::consts::PI;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Universe {
    width: usize,
    height: usize,
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
    pub fn new(width: usize, height: usize) -> Universe {
        utils::set_panic_hook();
        let max_tilt = 0.0;
        let dt = 0.1;

        // Create a new grid of the given size
        let quantum = Grid::<Complex>::new(width, height);
        let walls = Grid::<bool>::new(width, height);
        let sinks = Grid::<bool>::new(width, height);
        let sink_mult = Grid::<f32>::new(width, height);
        let potential_level = Grid::<f32>::new(width, height);
        let potential_cache = Grid::<f32>::new(width, height);

        Universe {
            width,
            height,
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
        // self.add_gaussian(Coord::new(8, 8), 1.0, 0.0, 0.0, 1.0);
        // self.add_potential_cone(4, 4, 3.0, 2.0);
        self.setup_sink_mult();
        self.setup_walls();
        self.ensure_no_positive_potential();
    }

    /// Compute the steps throught the quantum field theory.
    pub fn step(&mut self) {
        let dt = self.dt;
        let x_slope = 0.0;
        let y_slope = 0.0;
        self.reset_potential_cache(x_slope, y_slope);

        for y in 1..self.height - 1 {
            for x in 1..self.width - 1 {
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
                            + dt * (-0.5
                                * (top.re + bottom.re + left.re + right.re - 4.0 * cx.re)
                                + potential_cache * cx.re));
                    let im = sink_mult
                        * (cx.re
                            + dt * (-0.5
                                * (top.im + bottom.im + left.im + right.im - 4.0 * cx.im)
                                + potential_cache * cx.im));

                    self.quantum.set(coord, Complex::new(re, im));
                }
            }
        }
    }

    /// Retrieve cells for the web app.
    pub fn cells(&self) -> *const u8 {
        let mut cells = Vec::new();
        for cell in self.quantum.data.iter() {
            let color = cell.into_rgb();
            cells.push(color.r);
            cells.push(color.g);
            cells.push(color.b);
        }
        cells.as_ptr()
    }

    pub fn potential_level(&self) -> *const u8 {
        let mut cells = Vec::new();
        for cell in self.potential_level.data.iter() {
            let color = (255. - (cell * 255.)) as u8;
            cells.push(color);
            cells.push(color);
            cells.push(color);
        }
        cells.as_ptr()
    }

    pub fn potential_cache(&self) -> *const u8 {
        let mut cells = Vec::new();
        for cell in self.potential_cache.data.iter() {
            let color = (255. - (cell * 255.)) as u8;
            cells.push(color);
            cells.push(color);
            cells.push(color);
        }
        cells.as_ptr()
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
        for y in 0..self.height {
            for x in 0..self.width {
                let coord = Coord::new(x as i32, y as i32);
                if self.is_wall(coord) {
                    self.quantum.set(coord, Complex::new(0.0, 0.0));
                }
            }
        }
    }

    /// Flood fill sink_mult with 0 where not a sink; otherwise distance in pixels from non-sink
    /// ...basically a mini-Dijkstra
    fn setup_sink_mult(&mut self) {
        // let mut queue: PriorityQueue<Pixel, i32> = PriorityQueue::new();
        let queue = SegQueue::new();
        for y in 0..self.height {
            for x in 0..self.width {
                let coord = Coord::new(x as i32, y as i32);
                self.sink_mult.set(coord, f32::INFINITY);

                // Fill priority queue
                if !self.is_wall(coord) && !self.is_sink(coord) {
                    queue.push(Pixel::new(coord, 0));
                }
            }
        }

        while !queue.is_empty() {
            let p: Pixel = queue.pop().unwrap();
            if *self.sink_mult.get(p.coord).unwrap() > p.val as f32 {
                self.sink_mult.set(p.coord, p.val as f32);

                let neighbors = self.sink_mult.valid_neighbors(p.coord);
                for coord in neighbors {
                    let q: Pixel = Pixel::new(coord, p.val + 1);
                    queue.push(q);
                }
            }
        }

        //now convert these to actual sink_mults
        let suddenness: f32 = 0.005;
        for y in 0..self.height - 1 {
            for x in 0..self.width - 1 {
                let coord = Coord::new(x as i32, y as i32);

                let dist = self.sink_mult.get(coord).unwrap();
                let value = (-(dist / 2.0).powf(2.0) * suddenness).exp();
                self.sink_mult.set(coord, value);
            }
        }
    }

    /// Add a gaussian distribution to the quantum complex field
    pub fn add_gaussian(&mut self, c: Coord, sigma: f32, fx: f32, fy: f32, a_scale: f32) {
        let a: f32 = a_scale * (2.0 * PI * sigma * sigma).powf(-0.25);
        let d: f32 = 4.0 * sigma * sigma;
        let omega_x = 2.0 * PI * fx;
        let omega_y = 2.0 * PI * fy;

        let fwidth = self.width as f32;
        let fheight = self.height as f32;
        for x in 1..self.width - 1 {
            for y in 1..self.height - 1 {
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

    /// Add potential plane to the potential level field
    // pub fn addPotentialPlane(&self, tl: f32, tr: f32, bl: f32, br: f32, mask: Grid<bool>) {
    //     //find extremes
    //     let top = mask.height();
    //     let left = mask.width();
    //     let bottom = 0;
    //     let right = 0;

    //     for x in 0..mask.width() {
    //         for y in 0..mask.height() {
    //             if mask.get(x, y) {
    //                 if x < left {
    //                     left = x;
    //                 }
    //                 if x > right {
    //                     right = x;
    //                 }
    //                 if y < top {
    //                     top = y;
    //                 }
    //                 if y > bottom {
    //                     bottom = y;
    //                 }
    //             }
    //         }
    //     }
    //     //add potential
    //     let pot_width = right - left;

    //     if pot_width == 0 {
    //         pot_width = 1;
    //     }
    //     let pot_height = bottom - top;
    //     if pot_height == 0 {
    //         pot_height = 1;
    //     }
    //     for x in left..right {
    //         for y in top..bottom {
    //             if mask.get(x, y) {
    //                 let coord = Coord::new(x as i32, y as i32);

    //                 let rx = (x - left) / pot_width as f32;
    //                 let ry = (y - top) / pot_height as f32;
    //                 // potx0, potxh = potential at x,0 and x,height
    //                 let potx0 = tl + (tr - tl) * rx;
    //                 let potxh = bl + (br - bl) * rx;
    //                 let p = potx0 + (potxh - potx0) * ry;
    //                 self.potential_level.add(x, y, p);
    //             }
    //         }
    //     }
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
                    self.potential_level.add(coord, pot);
                }
            }
        }
    }

    /// Add potential well starting from a given point
    pub fn add_potential_well(&mut self, xc: i32, yc: i32, radius: f32, core_pot: f32) {
        let b: f32 = -core_pot / 3. / radius / radius;
        let a: f32 = 2.0 * b * radius * radius;
        for y in 0..self.height {
            for x in 0..self.width {
                let coord = Coord::new(x as i32, y as i32);
                let dx = x as i32 - xc;
                let dy = y as i32 - yc;
                let r: f32 = ((dx * dx + dy * dy) as f32).powf(0.5);
                if r < radius {
                    self.potential_level
                        .add(coord, b * (r * r - 3. * radius * radius));
                } else {
                    self.potential_level.add(coord, -a / r);
                }
            }
        }
    }

    /// Toggle cell at coord
    pub fn toggle_cell(&mut self, row: i32, column: i32) {
        // self.add_potential_cone(row, column, 5.0, 0.1);
        self.add_gaussian(Coord::new(column, row), 2.0, 0.0, 0.0, 1.0);
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
        for y in 0..self.height {
            left_edge_pot += y_pot_step;
            let mut current_pot = left_edge_pot;
            for x in 0..self.width {
                let coord = Coord::new(x as i32, y as i32);
                current_pot += x_pot_step;
                self.potential_cache.set(
                    coord,
                    current_pot + self.potential_level.get(coord).unwrap(),
                );
            }
        }
    }

    /// Ensure there is no positive potential
    /// Get max potential and subtract from all cells
    fn ensure_no_positive_potential(&mut self) {
        let max_pot = self.potential_level.max();

        for y in 0..self.height {
            for x in 0..self.width {
                let coord = Coord::new(x as i32, y as i32);
                self.potential_level.add(coord, -max_pot);
            }
        }
    }
}

#[cfg(test)]
#[test]
fn test_universe_creation() {
    let u = Universe::new(5, 5);
    assert_eq!(u.width, 5);
    assert_eq!(u.height, 5);
    assert_eq!(u.quantum.data.len(), 25);
}

#[test]
fn test_setup_walls_submask() {
    let mut u = Universe::new(5, 5);
    u.walls.set(Coord::new(1, 1), true);
    u.walls.set(Coord::new(2, 2), true);
    u.setup_walls();
    println!("{}", u.walls);
    assert_eq!(u.quantum.data.len(), 25);
}

#[test]
fn setup_sink_mult() {
    let mut u = Universe::new(5, 5);
    u.setup_sink_mult();
    println!("{}", u.sink_mult);
}

#[test]
/// test that the gaussian distribution is set correctly
fn adding_gaussian_to_quantum() {
    let mut u = Universe::new(10, 10);
    u.add_gaussian(Coord::new(4, 4), 1.2, 0., 0., 1.);
    u.add_gaussian(Coord::new(8, 8), 1.0, 0., 0., 0.7);
    println!("{}", u.quantum);
}

#[test]
fn adding_potential_cone() {
    let mut u = Universe::new(5, 5);
    u.add_potential_cone(2, 2, 3.0, 1.0);
    u.ensure_no_positive_potential();
    println!("{}", u.potential_level);
    println!("{:?}", u.potential_level.data);
}

#[test]
fn adding_potential_well() {
    let mut u = Universe::new(5, 5);
    u.add_potential_well(2, 2, 3.0, 1.0);
    u.ensure_no_positive_potential();
    println!("{}", u.potential_level);
    println!("{:?}", u.potential_level.data);
}

#[test]
fn adding_potential_well_and_cone() {
    let mut u = Universe::new(5, 5);
    u.add_potential_cone(2, 2, 3.0, 1.0);
    u.add_potential_well(2, 2, 3.0, 1.0);
    u.ensure_no_positive_potential();
    println!("{}", u.potential_level);
    println!("{:?}", u.potential_level.data);
}

#[test]
fn reset_potential_cache() {
    let mut u = Universe::new(5, 5);
    u.add_potential_cone(2, 2, 3.0, 1.0);
    u.add_potential_well(2, 2, 3.0, 1.0);
    u.ensure_no_positive_potential();
    u.reset_potential_cache(0.0, 0.0);
    println!("{}", u.potential_cache);
    println!("{:?}", u.potential_cache.data);
}

#[test]
fn setup() {
    let mut u = Universe::new(5, 5);
    u.setup();
    println!("{}", u.quantum);
}

#[test]
fn steps() {
    let mut u = Universe::new(5, 5);
    u.setup();
    u.add_gaussian(Coord::new(2, 2), 1.0, 0.0, 0.0, 1.0);
    for _i in 0..20 {
        u.step();
        println!("{}", u.quantum);
        println!("{:?}", u.quantum.data);
    }
}
