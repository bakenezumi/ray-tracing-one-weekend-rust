use std::ops::Neg;
use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;
use std::ops::Range;
use std::f64::consts::PI;
use std::fmt;
use rand::Rng;
use rand::rngs::ThreadRng;

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

impl Mul<Vec3> for f64 {
  type Output = Vec3;
  fn mul(self, t: Vec3) -> Self::Output {
    Vec3 {
      x: self * t.x,
      y: self * t.y,
      z: self * t.z
    }
  }
}

impl Mul<Vec3> for Vec3 {
  type Output = Vec3;
  fn mul(self, v: Vec3) -> Self::Output {
    Vec3 {
      x: self.x * v.x,
      y: self.y * v.y,
      z: self.z * v.z
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
  pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
    Vec3 { x, y, z }
  }

  pub fn zero() -> Vec3 {
    Vec3 { x: 0.0, y: 0.0, z: 0.0 }
  }

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

  pub fn random(rng: &mut ThreadRng) -> Vec3 {
    Vec3 {
      x: rng.gen(),
      y: rng.gen(),
      z: rng.gen()
    }
  }

  pub fn random_range(rng: &mut ThreadRng, range: Range<f64>) -> Vec3 {
    Vec3 {
      x: rng.gen_range(range.clone()),
      y: rng.gen_range(range.clone()),
      z: rng.gen_range(range)
    }
  }

  pub fn random_in_unit_sphere(rng: &mut ThreadRng) -> Vec3 {
    loop {
      let p = Vec3::random_range(rng, -1.0..1.0);
      if p.length_squared() >= 1.0 {
        continue;
      }
      return p;
    }
  }

  pub fn random_unit_vector(rng: &mut ThreadRng) -> Vec3 {
    let a: f64 = rng.gen_range(0.0..2.0*PI);
    let z: f64 = rng.gen_range(-1.0..1.0);
    let r: f64 = (1.0 - z*z).sqrt();
    Vec3 {
      x: r*a.cos(),
      y: r*a.sin(),
      z: z
    }
  }

  pub fn random_in_hemisphere(rng: &mut ThreadRng, normal: &Vec3) -> Vec3 {
    let in_unit_sphere = Vec3::random_in_unit_sphere(rng);
    if in_unit_sphere.dot(normal) > 0.0 {
      in_unit_sphere
    } else {
      -in_unit_sphere
    }
  }

  pub fn reflect(&self, n: &Vec3) -> Vec3 {
    *self - *n*self.dot(n)*2.0
  }

  pub fn refract(&self, n: &Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = -self.dot(n);
    let r_out_parallel =   (*self + *n*cos_theta) * etai_over_etat;
    let r_out_perp = *n * -(1.0 - r_out_parallel.length_squared()).sqrt();
    r_out_parallel + r_out_perp
  }

   pub fn random_in_unit_disk(rng: &mut ThreadRng) -> Vec3 {
    let mut p: Vec3;
    loop {
      p = Vec3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0);
      if p.length_squared() >= 1.0 {
        continue;
      }
      return p;
    }
  }

  pub fn d(&self, i: i32) -> f64 {
    if i == 0 {
      self.x
    } else if i == 1 {
      self.y
    } else { // FIXME
      self.z
    }
  }

}

pub type Point3 = Vec3;
pub type Color = Vec3;

impl Color {
  pub fn black() -> Color {
    Point3 { x: 0.0, y: 0.0, z: 0.0 }
  }
}