use core::panic;
use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

#[derive(Default, Debug, Clone, Copy)]
pub struct Vector3(f64, f64, f64);

impl Vector3 {
    pub fn new() -> Self {
        Self(0.0, 0.0, 0.0)
    }

    pub fn x(&self) -> f64 {
        self.0
    }

    pub fn y(&self) -> f64 {
        self.1
    }

    pub fn z(&self) -> f64 {
        self.2
    }

    pub fn dot(&self, rhs: Self) -> f64 {
        self.0 * rhs.0 + self.1 * rhs.1 + self.2 * rhs.2
    }

    pub fn cross(&self, rhs: Self) -> Self {
        Self::from((
            self.1 * rhs.2 - self.2 * rhs.1,
            self.2 * rhs.0 - self.0 * rhs.2,
            self.0 * rhs.1 - self.1 * rhs.0,
        ))
    }

    pub fn hadamard(&self, rhs: Self) -> Self {
        Self::from((self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2))
    }

    pub fn length(&self) -> f64 {
        self.dot(*self).sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.dot(*self)
    }

    pub fn normalized(&self) -> Self {
        *self / self.length()
    }
}

impl From<(f64, f64, f64)> for Vector3 {
    fn from(value: (f64, f64, f64)) -> Self {
        Self(value.0, value.1, value.2)
    }
}

impl Index<usize> for Vector3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            x => panic!("Invalid Index {x} accessed!"),
        }
    }
}

impl IndexMut<usize> for Vector3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.0,
            1 => &mut self.1,
            2 => &mut self.2,
            x => panic!("Invalid Index {x} accessed!"),
        }
    }
}

impl Add for Vector3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::from((self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2))
    }
}

impl AddAssign for Vector3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self::from((self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2));
    }
}

impl Neg for Vector3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::from((-self.0, -self.1, -self.2))
    }
}

impl Sub for Vector3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

impl SubAssign for Vector3 {
    fn sub_assign(&mut self, rhs: Self) {
        *self += -rhs;
    }
}

impl Mul<f64> for Vector3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::from((self.0 * rhs, self.1 * rhs, self.2 * rhs))
    }
}

impl MulAssign<f64> for Vector3 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = Self::from((self.0 * rhs, self.1 * rhs, self.2 * rhs));
    }
}

impl Div<f64> for Vector3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl DivAssign<f64> for Vector3 {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1.0 / rhs;
    }
}

pub type Point3D = Vector3;
