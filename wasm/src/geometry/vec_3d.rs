use std::ops::{Add, AddAssign, Sub};

#[derive(Copy, Clone)]
pub struct V3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl AddAssign for V3 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}


impl Add for V3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(
            self.x+other.x,
            self.y+other.y,
            self.z+other.z)
    }
}

impl Sub for V3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::new(
            self.x-other.x,
            self.y-other.y,
            self.z-other.z)
    }
}


impl V3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {x, y, z}
    }
    pub fn null() -> Self {
        Self {x: 0.0, y: 0.0, z: 0.0}
    }
    pub fn scale(self, k: f32) -> Self {
        Self {x: self.x * k, y: self.y * k, z: self.z * k}
    }
    pub fn normalize(self) -> Self {
        let r = fast_inverse_square_root(
            self.x*self.x+
            self.y*self.y+
            self.z*self.z);

        self.scale(r)
    }
    pub fn cross(a: V3, b: V3) -> V3 {
        V3::new(
            a.y*b.z - a.z*b.y,
            a.z*b.x - a.x*b.z,
            a.x*b.y - a.y*b.x,
            )
    }
    pub fn dot(a: V3, b: V3) -> f32 {
        a.x*b.x+a.y*b.y+a.z*b.z
    }
}


fn fast_inverse_square_root(x: f32) -> f32 {
    let i = x.to_bits();
    let i = 0x5f3759df - (i >> 1);
    let y = f32::from_bits(i);

    y * (1.5 - 0.5 * x * y * y)
}
