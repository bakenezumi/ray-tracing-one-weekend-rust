use std::ops::Neg;
use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;
use std::fmt;

pub struct Vec3 {
  pub x: f64,
  pub y: f64,
  pub z: f64
}

impl Copy for Vec3 {}

impl Clone for Vec3 {
  fn clone(&self) -> Vec3 {
    *self
  }
}


impl Neg for Vec3 {
  type Output = Vec3;
  fn neg(self) -> Self::Output {
    Vec3 {
      x: -self.x,
      y: -self.y,
      z: -self.z
    }
  }
}

impl Add for Vec3 {
  type Output = Vec3;
  fn add(self, rhs: Self) -> Self::Output {
    Vec3 {
      x: self.x + rhs.x,
      y: self.y + rhs.y,
      z: self.z + rhs.z
    }
  }
}

impl Sub for Vec3 {
  type Output = Vec3;
  fn sub(self, rhs: Self) -> Self::Output {
    Vec3 {
      x: self.x - rhs.x,
      y: self.y - rhs.y,
      z: self.z - rhs.z
    }
  }
}

impl Mul<f64> for Vec3 {
  type Output = Vec3;
  fn mul(self, t: f64) -> Self::Output {
    Vec3 {
      x: self.x * t,
      y: self.y * t,
      z: self.z * t
    }
  }
}

impl Div<f64> for Vec3 {
  type Output = Vec3;
  fn div(self, t: f64) -> Self::Output {
    self * (1.0 / t)
  }
}

impl fmt::Display for Vec3 {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "{} {} {}", self.x, self.y, self.z)
  }
}

impl Vec3 {
  pub fn length(&self) -> f64 {
    self.length_squared().sqrt()
  }

  pub fn length_squared(&self) -> f64 {
    self.x * self.x + self.y * self.y + self.z * self.z
  }

  pub fn dot(&self, v: &Vec3) -> f64 {
    self.x * v.x + self.y * v.y + self.z * v.z
  }

  pub fn cross(&self, v: &Vec3) -> Vec3 {
    Vec3 {
      x: self.y * v.z - self.z * v.y,
      y: self.z * v.x - self.x * v.z,
      z: self.x * v.y - self.y * v.x
    }
  }

  pub fn unit_vector(&self) -> Vec3 {
    *self / self.length()
  }
}

pub type Point3 = Vec3;
pub type Color = Vec3;
