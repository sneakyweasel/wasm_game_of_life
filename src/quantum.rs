// use num::complex::Complex;
// use priority_queue::PriorityQueue;
// use std::f32::consts::PI;
// use std::fmt;

// mod arrays;

// #[derive(PartialEq, Eq, Hash)]
// pub struct Pixel {
//     x: i32,
//     y: i32,
//     d: i32,
// }

// impl Pixel {
//     pub fn new(x: i32, y: i32, d: i32) -> Self {
//         Pixel { x, y, d }
//     }
// }

// #[derive(Clone, Debug)]
// pub struct QuantumData {
//     height: usize,
//     width: usize,
//     pub real: FloatArray2D,
//     pub imag: FloatArray2D,
//     init_real: FloatArray2D,
//     init_imag: FloatArray2D,
//     pub walls: BoolArray2D,
//     pub sink: BoolArray2D,
//     pub sink_mult: FloatArray2D,
//     pub level_design_potential: FloatArray2D,
//     pub potential_cache: FloatArray2D,
//     counters: Vec<BoolArray2D>,
//     delta_t: f32,
//     max_tilt: f32,
//     running: bool,
// }

// impl QuantumData {
//     pub fn new(width: usize, height: usize) -> QuantumData {
//         QuantumData {
//             height: height,
//             width: width,
//             real: FloatArray2D::new(width, height),
//             imag: FloatArray2D::new(width, height),
//             init_real: FloatArray2D::new(width, height),
//             init_imag: FloatArray2D::new(width, height),
//             walls: BoolArray2D::new(width, height),
//             sink: BoolArray2D::new(width, height),
//             sink_mult: FloatArray2D::new(width, height),
//             level_design_potential: FloatArray2D::new(width, height),
//             potential_cache: FloatArray2D::new(width, height),
//             counters: vec![],
//             delta_t: 0.0,
//             max_tilt: 0.0,
//             running: false,
//         }
//     }

//     fn save_initial_state(self) {
//         self.init_real.set_equal_to(self.real);
//         self.init_imag.set_equal_to(self.imag);
//     }

//     fn reset_initial_state(self) {
//         self.real.set_equal_to(self.init_real);
//         self.imag.set_equal_to(self.init_imag);
//     }

//     fn add_delta(&mut self, xc: usize, yc: usize, a: f32) {
//         self.real.set(xc, yc, a);
//         self.imag.set(xc, yc, a);
//     }

//     fn add_counter(&mut self, sub_mask: BoolArray2D) {
//         self.counters.push(sub_mask)
//     }

//     pub fn init(&mut self) {
//         self.setup_sink_mult();
//         self.add_walls();
//         // self.save_initial_state();
//         self.ensure_no_positive_potential();
//     }

//     pub fn step(&mut self) {
//         // initialisation
//         if !self.running {
//             self.running = true;
//             self.init();
//         }
//         // running
//         // self.control_state.step():
//         let x_slope = 0.0;
//         let y_slope = 0.0;
//         self.reset_potential_cache(x_slope, y_slope);

//         // Real step
//         for y in 1..self.height - 1 {
//             for x in 1..self.width - 1 {
//                 if !self.walls.get(x, y) {
//                     let value = self.sink_mult.get(x, y)
//                         * (self.real.get(x, y)
//                             + self.delta_t
//                                 * (-0.5
//                                     * (self.imag.get(x, y - 1)
//                                         + self.imag.get(x, y + 1)
//                                         + self.imag.get(x - 1, y)
//                                         + self.imag.get(x + 1, y)
//                                         - 4.0 * self.imag.get(x, y))
//                                     + self.potential_cache.get(x, y) * self.imag.get(x, y)));
//                     self.real.set(x, y, value)
//                 }
//             }
//         }
//         // Imaginary step
//         for y in 1..self.height - 1 {
//             for x in 1..self.width - 1 {
//                 if !self.walls.get(x, y) {
//                     let value = self.sink_mult.get(x, y)
//                         * (self.imag.get(x, y)
//                             + self.delta_t
//                                 * (-0.5
//                                     * (self.real.get(x, y - 1)
//                                         + self.real.get(x, y + 1)
//                                         + self.real.get(x - 1, y)
//                                         + self.real.get(x + 1, y)
//                                         - 4.0 * self.real.get(x, y))
//                                     + self.potential_cache.get(x, y) * self.real.get(x, y)));
//                     self.imag.set(x, y, value)
//                 }
//             }
//         }
//     }
// }

// #[derive(Clone, Debug)]
// pub struct Game {
//     pub data: Vec<f32>,
//     pub qd: QuantumData,
//     color_map: ColorMap,
// }

// impl Game {
//     pub fn new(qd: QuantumData) -> Game {
//         Game {
//             qd,
//             data: Vec::new(),
//             color_map: ColorMap::new(),
//         }
//     }
//     pub fn width(&self) -> usize {
//         self.qd.width
//     }

//     pub fn height(&self) -> usize {
//         self.qd.height
//     }

//     pub fn set_amp_color_map(&mut self) {
//         self.color_map = ColorMap::new();
//     }

//     // pub fn update(&mut self) {
//     //     let show_potential = false;
//     //     for y in 0..self.qd.height {
//     //         for x in 0..self.qd.width {
//     //             let point: Complex = if show_potential {
//     //                 Complex::new(0.0, self.qd.get_potential(x, y))
//     //             } else {
//     //                 self.qd.get(x, y)
//     //             };
//     //             self.data[x + self.qd.width * y] = self.color_map.process(point);
//     //         }
//     //     }
//     // }
// }
