use std::f32::consts::PI;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Complex {
    pub re: f32,
    pub im: f32,
}

impl Complex {
    pub fn zero() -> Complex {
        Complex { re: 0.0, im: 0.0 }
    }

    pub fn new(re: f32, im: f32) -> Self {
        Complex { re, im }
    }

    pub fn conj(&self) -> Self {
        Complex {
            re: self.re,
            im: -self.im,
        }
    }

    pub fn norm(&self) -> f32 {
        self.re * self.re + self.im * self.im
    }

    pub fn to_polar(&self) -> (f32, f32) {
        (self.norm(), self.arg())
    }

    pub fn from_polar(r: f32, theta: f32) -> Self {
        Complex {
            re: r * theta.cos(),
            im: r * theta.sin(),
        }
    }

    pub fn arg(&self) -> f32 {
        self.im.atan2(self.re)
    }

    pub fn phi(&self) -> f32 {
        self.arg()
    }

    pub fn radius(&self) -> f32 {
        self.re.hypot(self.im)
    }

    pub fn add(&self, other: &Complex) -> Self {
        Complex {
            re: self.re + other.re,
            im: self.im + other.im,
        }
    }

    pub fn sub(&self, other: &Complex) -> Self {
        Complex {
            re: self.re - other.re,
            im: self.im - other.im,
        }
    }

    pub fn mul(&self, other: &Complex) -> Self {
        Complex {
            re: self.re * other.re - self.im * other.im,
            im: self.re * other.im + self.im * other.re,
        }
    }

    pub fn to_hsl(&self) -> (u8, u8, u8) {
        let hue = ((self.phi() * 360.0 / (2.0 * PI)) % 360.0) as u8;
        let saturation = if self.radius() == 0.0 { 0 } else { 100 };
        let lightness = (100.0 - (self.radius() * 50.0)) as u8;
        (hue, saturation, lightness)
    }

    pub fn to_string(&self) -> String {
        format!("{} + {}i", self.re, self.im)
    }
}
