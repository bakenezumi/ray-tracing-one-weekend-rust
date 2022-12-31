use std::ops::Neg;
use std::ops::Add;
use std::ops::Mul;
use std::ops::Div;
use std::fmt;

struct Vec3 {
  x: f64,
  y: f64,
  z: f64
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
  fn length(&self) -> f64 {
    self.length_squared().sqrt()
  }

  fn length_squared(&self) -> f64 {
    self.x * self.x + self.y * self.y + self.z * self.z
  }

  fn dot(&self, v: &Vec3) -> f64 {
    self.x * v.x + self.y * v.y + self.z * v.z
  }

  fn cross(&self, v: &Vec3) -> Vec3 {
    Vec3 {
      x: self.y * v.z - self.z * v.y,
      y: self.z * v.x - self.x * v.z,
      z: self.x * v.y - self.y * v.x
    }
  }

  fn unit_vector(&self) -> Vec3 {
    *self / self.length()
  }
}

type Point3 = Vec3;
type Color = Vec3;


fn write_color(color: &Color) {
  fn f(v: f64) -> i64 {
    (v * 255.999) as i64
  }
  println!("{} {} {}", f(color.x), f(color.y), f(color.z));
}

fn main() {
  let image_width = 256;
  let image_height = 256; 

  println!("P3");
  println!("{} {}", image_width, image_height);
  println!("255");
  for j in 0 .. image_height {
    eprint!("\rScanlines remaining: {} ", j);
    for i in 0 .. image_width {
      let r = (i as f64) / (image_width) as f64;
      let g = (j as f64) / (image_height) as f64;
      let b = 0.25;
      let pixel_color = Vec3 { x: r, y: g, z: b };      
      write_color(&pixel_color);    
    }
  }
  eprintln!("\nDone.");
}