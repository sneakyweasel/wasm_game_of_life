use color::Color;
use colorsys::*;
use std::f32::consts::PI;
use std::fmt;

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

    pub fn into_rgb(&self) -> Color {
        let h = (((self.phi() * 180. / PI) + 360.) % 360.) as f64;
        let s = if self.radius() == 0. { 0. } else { 100. };
        let l = (self.radius() * 50.0)) as f64;
        let hsl = Hsl::from((h, s, l));
        let rgb: Rgb = Rgb::from(&hsl);
        Color {
            r: 255 - rgb.red() as u8,
            g: 255 - rgb.green() as u8,
            b: 255 - rgb.blue() as u8,
        }
    }
}

impl Default for Complex {
    fn default() -> Self {
        Complex::zero()
    }
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{} + {}i", self.re, self.im)
    }
}

#[cfg(test)]
#[test]
fn display() {
    let cx = Complex::new(1.0, 2.0);
    assert_eq!(format!("{}", cx), "1 + 2i");
}

#[test]
fn into_rgb() {
    let cx = Complex::new(0.4, -0.2);
    let color = cx.into_rgb();
    assert_eq!(
        color,
        Color {
            r: 0,
            g: 115,
            b: 64
        }
    );
}
