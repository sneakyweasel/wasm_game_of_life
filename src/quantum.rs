use num::complex::Complex;
use priority_queue::PriorityQueue;
use std::f32::consts::PI;
use std::fmt;

#[derive(Clone, Debug)]
pub struct FloatArray2D {
    data: Vec<f32>,
    width: usize,
}

impl FloatArray2D {
    pub fn new(width: usize, height: usize) -> Self {
        let data: Vec<f32> = vec![0.0; width * height];
        FloatArray2D { data, width }
    }
    pub fn set_equal_to(&self, other: FloatArray2D) -> Self {
        FloatArray2D {
            width: other.width,
            data: other.data.clone(),
        }
    }
    pub fn get(&self, x: usize, y: usize) -> f32 {
        self.data[x + self.width * y]
    }
    pub fn set(&mut self, x: usize, y: usize, value: f32) {
        self.data[x + self.width * y] = value
    }
    pub fn del2(&self, x: usize, y: usize) -> f32 {
        self.get(x, y - 1) + self.get(x, y + 1) + self.get(x - 1, y) + self.get(x + 1, y)
            - 4.0 * self.get(x, y)
    }
}

impl fmt::Display for FloatArray2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.data)
    }
}

#[derive(Clone, Debug)]
pub struct ByteArray2D {
    data: Vec<u8>,
    width: usize,
}

impl ByteArray2D {
    pub fn new(width: usize, height: usize) -> Self {
        let data: Vec<u8> = vec![0; width * height];
        ByteArray2D { data, width }
    }
    pub fn get(&self, x: usize, y: usize) -> u8 {
        self.data[x + self.width * y]
    }
    pub fn set(&mut self, x: usize, y: usize, value: u8) {
        self.data[x + self.width * y] = value
    }
    pub fn height(&self) -> usize {
        self.data.len() / self.width
    }
}

#[derive(Clone, Debug)]
pub struct BoolArray2D {
    data: Vec<bool>,
    width: usize,
}

impl BoolArray2D {
    pub fn new(width: usize, height: usize) -> Self {
        let data: Vec<bool> = vec![false; width * height];
        BoolArray2D { data, width }
    }
    pub fn get(&self, x: usize, y: usize) -> bool {
        self.data[x + self.width * y]
    }
    pub fn set(&mut self, x: usize, y: usize, value: bool) {
        self.data[x + self.width * y] = value
    }
    pub fn height(&self) -> usize {
        self.data.len() / self.width
    }
}

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

#[derive(Clone, Debug)]
pub struct QuantumData {
    height: usize,
    width: usize,
    pub real: FloatArray2D,
    pub imag: FloatArray2D,
    init_real: FloatArray2D,
    init_imag: FloatArray2D,
    pub walls: BoolArray2D,
    pub sink: BoolArray2D,
    pub sink_mult: FloatArray2D,
    pub level_design_potential: FloatArray2D,
    pub potential_cache: FloatArray2D,
    counters: Vec<BoolArray2D>,
    delta_t: f32,
    max_tilt: f32,
    running: bool,
}

impl QuantumData {
    pub fn new(width: usize, height: usize) -> QuantumData {
        QuantumData {
            height: height,
            width: width,
            real: FloatArray2D::new(width, height),
            imag: FloatArray2D::new(width, height),
            init_real: FloatArray2D::new(width, height),
            init_imag: FloatArray2D::new(width, height),
            walls: BoolArray2D::new(width, height),
            sink: BoolArray2D::new(width, height),
            sink_mult: FloatArray2D::new(width, height),
            level_design_potential: FloatArray2D::new(width, height),
            potential_cache: FloatArray2D::new(width, height),
            counters: vec![],
            delta_t: 0.0,
            max_tilt: 0.0,
            running: false,
        }
    }

    fn save_initial_state(self) {
        self.init_real.set_equal_to(self.real);
        self.init_imag.set_equal_to(self.imag);
    }

    fn reset_initial_state(self) {
        self.real.set_equal_to(self.init_real);
        self.imag.set_equal_to(self.init_imag);
    }

    fn clear_wave_function(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                self.real.set(x, y, 0.0);
                self.imag.set(x, y, 0.0);
            }
        }
    }

    fn set_delta_t(mut self, dt: f32) {
        self.delta_t = dt;
    }

    fn set_max_tilt(mut self, mt: f32) {
        self.max_tilt = mt;
    }

    pub fn add_gaussian(&mut self, xc: i32, yc: i32, sigma: f32, fx: f32, fy: f32, a_scale: f32) {
        let a: f32 = a_scale * 2.0 * PI;
        let d: f32 = 4.0 * sigma * sigma;
        let omega_x = 2.0 * PI * fx; // seems wrong
        let omega_y = 2.0 * PI * fy; // seems wrong

        for x in 1..&self.width - 1 {
            for y in 1..&self.height - 1 {
                let r2 =
                    ((x as i32) - xc) * ((x as i32) - xc) + ((y as i32) - yc) * ((y as i32) - yc);
                let v_real = a
                    * f32::exp(-(r2 as f32) / d)
                    * (omega_x * (x as f32) / (self.width as f32)).cos()
                    * (omega_y * (y as f32) / (self.height as f32)).cos();
                let v_imag = a
                    * f32::exp(-(r2 as f32) / d)
                    * (omega_x * (x as f32) / (self.width as f32)).sin()
                    * (omega_y * (y as f32) / (self.height as f32)).sin();
                let real_val = self.real.get(x, y) + v_real;
                let imag_val = self.imag.get(x, y) + v_imag;
                self.real.set(x, y, real_val);
                self.imag.set(x, y, imag_val);
            }
        }
    }

    fn add_delta(&mut self, xc: usize, yc: usize, a: f32) {
        self.real.set(xc, yc, a);
        self.imag.set(xc, yc, a);
    }

    fn get(&self, x: usize, y: usize) -> Complex {
        Complex::new(self.real.get(x, y), self.imag.get(x, y))
    }

    fn ensure_no_positive_potential(&mut self) {
        let mut max_pot = f32::NEG_INFINITY;
        for x in 0..self.width {
            for y in 0..self.height {
                let pot = self.level_design_potential.get(x, y);
                if pot > max_pot {
                    max_pot = pot;
                }
            }
        }
        for x in 0..self.width {
            for y in 0..self.height {
                let pot = self.level_design_potential.get(x, y) - max_pot;
                self.level_design_potential.set(x, y, pot);
            }
        }
    }

    fn add_walls(&mut self) {
        for x in 1..self.width {
            for y in 1..self.height {
                if self.walls.get(x, y) {
                    self.real.set(x, y, 0.0);
                    self.imag.set(x, y, 0.0);
                }
            }
        }
    }

    //potentials >0 are problematic
    //pixel wide band with potential +1 above background - tunnelling
    //potential of -5 over width of universe - good for steering
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

        //adjust all potentials to be <0
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

    fn get_potential(&self, x: usize, y: usize) -> f32 {
        self.level_design_potential.get(x, y)
    }

    fn add_potential(&mut self, x: usize, y: usize, pot: f32) {
        let orig = self.get_potential(x, y);
        self.level_design_potential.set(x, y, orig + pot)
    }

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

    fn set_walls(&mut self, sub_mask: BoolArray2D) {
        assert!(sub_mask.width == self.width);
        assert!(sub_mask.height() == self.height);
        self.walls = sub_mask
    }

    fn set_sink(&mut self, sub_mask: BoolArray2D) {
        assert!(sub_mask.width == self.width);
        assert!(sub_mask.height() == self.height);
        self.sink = sub_mask
    }

    fn add_counter(&mut self, sub_mask: BoolArray2D) {
        self.counters.push(sub_mask)
    }

    //flood fill sink_mult with 0 where not a sink; otherwise distance in pixels from non-sink
    //...basically a mini-Dijkstra
    fn setup_sink_mult(&mut self) {
        let mut queue: PriorityQueue<Pixel, i32> = PriorityQueue::new();
        for y in 0..self.height {
            for x in 0..self.width {
                let priority = (x * self.width + y) as i32;
                self.sink_mult.set(x, y, f32::INFINITY);
                if !self.sink.get(x, y) && !self.walls.get(x, y) {
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

    pub fn init(&mut self) {
        self.setup_sink_mult();
        self.add_walls();
        // self.save_initial_state();
        self.ensure_no_positive_potential();
    }

    pub fn step(&mut self) {
        // initialisation
        if !self.running {
            self.running = true;
            self.init();
        }
        // running
        // self.control_state.step():
        let x_slope = 0.0;
        let y_slope = 0.0;
        self.reset_potential_cache(x_slope, y_slope);

        // Real step
        for y in 1..self.height - 1 {
            for x in 1..self.width - 1 {
                if !self.walls.get(x, y) {
                    let value = self.sink_mult.get(x, y)
                        * (self.real.get(x, y)
                            + self.delta_t
                                * (-0.5
                                    * (self.imag.get(x, y - 1)
                                        + self.imag.get(x, y + 1)
                                        + self.imag.get(x - 1, y)
                                        + self.imag.get(x + 1, y)
                                        - 4.0 * self.imag.get(x, y))
                                    + self.potential_cache.get(x, y) * self.imag.get(x, y)));
                    self.real.set(x, y, value)
                }
            }
        }
        // Imaginary step
        for y in 1..self.height - 1 {
            for x in 1..self.width - 1 {
                if !self.walls.get(x, y) {
                    let value = self.sink_mult.get(x, y)
                        * (self.imag.get(x, y)
                            + self.delta_t
                                * (-0.5
                                    * (self.real.get(x, y - 1)
                                        + self.real.get(x, y + 1)
                                        + self.real.get(x - 1, y)
                                        + self.real.get(x + 1, y)
                                        - 4.0 * self.real.get(x, y))
                                    + self.potential_cache.get(x, y) * self.real.get(x, y)));
                    self.imag.set(x, y, value)
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct ColorMap {
    pub lookup: [f32; 256],
    pub max_index: usize,
    pub max: f32,
    pub gain: f32,
}

impl ColorMap {
    pub fn new() -> ColorMap {
        let max_index = 255;
        let gamma: f32 = 0.7;
        let mut lookup: [f32; 256] = [0.0; 256];
        for i in 0..max_index + 1 {
            lookup[i] = ((255 * i / max_index) as f32).powf(gamma);
        }
        ColorMap {
            lookup,
            max: 0.0,
            max_index: 255,
            gain: 0.0,
        }
    }

    pub fn process(mut self, c: Complex) -> f32 {
        let source = c.mod2();
        if source > self.max {
            self.max = source
        }
        let mut index: usize = (source * self.gain) as usize;
        if index > self.max_index {
            index = self.max_index;
        }
        self.lookup[index]
    }

    pub fn reset_gain(mut self) {
        self.gain = self.max_index as f32 / self.max as f32
    }
}

#[derive(Clone, Debug)]
pub struct Game {
    pub data: Vec<f32>,
    pub qd: QuantumData,
    color_map: ColorMap,
}

impl Game {
    pub fn new(qd: QuantumData) -> Game {
        Game {
            qd,
            data: Vec::new(),
            color_map: ColorMap::new(),
        }
    }
    pub fn width(&self) -> usize {
        self.qd.width
    }

    pub fn height(&self) -> usize {
        self.qd.height
    }

    pub fn set_amp_color_map(&mut self) {
        self.color_map = ColorMap::new();
    }

    // pub fn update(&mut self) {
    //     let show_potential = false;
    //     for y in 0..self.qd.height {
    //         for x in 0..self.qd.width {
    //             let point: Complex = if show_potential {
    //                 Complex::new(0.0, self.qd.get_potential(x, y))
    //             } else {
    //                 self.qd.get(x, y)
    //             };
    //             self.data[x + self.qd.width * y] = self.color_map.process(point);
    //         }
    //     }
    // }
}
