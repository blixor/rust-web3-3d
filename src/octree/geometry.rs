use std::ops::{Add, Mul};


#[derive(Copy, Clone)]
pub struct V3(pub f32, pub f32, pub f32);

impl Mul<f32> for V3 {
  type Output = Self;
  fn mul (self, r: f32) -> Self::Output { V3(self.0*r, self.1*r, self.2*r)}
}

impl Add for V3 {
  type Output = Self;
  fn add(self, other: V3) -> Self::Output { V3(self.0 + other.0, self.1 + other.1, self.2 + other.2) }
}


impl V3 {
  pub fn norm(self) -> f32 {(self.0*self.0 + self.1+self.1 + self.2+self.2).sqrt()}
}



pub trait Dist {
  fn dist(&self, p: V3) -> f32;
}

pub struct Sphere{center: V3, r: f32}

impl Sphere {
  pub fn new(center: V3, r: f32) -> Self { Self {center, r}}
}

impl Dist for Sphere {
  fn dist(&self, p: V3) -> f32 {
    (p + self.center*-1.0).norm() - self.r
  }
}
