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
}
