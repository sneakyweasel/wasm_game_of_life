use complex::Complex;

#[derive(Clone, Debug, PartialEq)]
pub struct ComplexField {
    pub width: usize,
    data: Vec<Complex>,
}

impl ComplexField {
    pub fn new(width: usize, height: usize) -> Self {
        let data: Vec<Complex> = vec![Complex::ZERO(); width * height];
        ComplexField { width, data }
    }

    pub fn get(&self, x: usize, y: usize) -> &Complex {
        &self.data[x + self.width * y]
    }

    pub fn set(&mut self, x: usize, y: usize, value: Complex) {
        self.data[x + self.width * y] = value
    }

    pub fn reset(&mut self) {
        for i in 0..self.data.len() {
            self.data[i] = Complex::ZERO();
        }
    }

    pub fn height(&self) -> usize {
        self.data.len() / self.width
    }

    pub fn process(&mut self, x: usize, y: usize, sink_mult: f32, dt: f32, potential_cache: f32) {
        let cx = self.get(x, y);
        let top = self.get(x, y - 1);
        let bottom = self.get(x, y + 1);
        let left = self.get(x - 1, y);
        let right = self.get(x + 1, y);

        let re = sink_mult
            * (cx.im
                + dt * (-0.5 * (top.re + bottom.re + left.re + right.re - 4.0 * cx.re)
                    + potential_cache * cx.re));
        let im = sink_mult
            * (cx.re
                + dt * (-0.5 * (top.im + bottom.im + left.im + right.im - 4.0 * cx.im)
                    + potential_cache * cx.im));

        self.set(x, y, Complex::new(re, im));
    }
}
