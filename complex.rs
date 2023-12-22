use std::ops::{Add, Mul};

#[derive(Clone, Copy)]
pub struct Complex {
   pub re: f64,
   pub im: f64,
}

impl Complex {
    pub fn new(re: f64, im: f64) -> Complex {
        Self { re, im }
    }

    pub fn mag(&self) -> f64 {
        self.re * self.re + self.im * self.im
    }
}

impl Add for Complex {
    type Output = Self;

     fn add(self, rs: Self) -> Self {
        Self {
            re: self.re + rs.re,
            im: self.im + rs.im,
        }
    }
}

impl Mul for Complex {
    type Output = Self;

    fn mul(self, rs: Self) -> Self {
        Self {
            re: self.re * rs.re - self.im * rs.im,
            im: self.re * rs.im + self.im * rs.re,
        }
    }
}