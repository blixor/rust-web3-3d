use std::ops::{Add, AddAssign, Sub};

#[derive(Copy, Clone, Debug)]
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

impl From<(f32, f32, f32)> for V3 {
    fn from(t: (f32, f32, f32)) -> V3 {
        Self {x: t.0, y: t.1, z: t.2}
    }
}
impl From<V3> for (f32, f32, f32) {
    fn from(p: V3) -> (f32, f32, f32) {
        (p.x, p.y, p.z)
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
    pub fn norm(self) -> f32 {
        V3::dot(self, self).sqrt()
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
    pub fn map(&self, f: impl Fn(f32) -> f32) -> Self{
        // consume the vector
        Self::new(f(self.x), f(self.y), f(self.z))
    }
}


fn fast_inverse_square_root(x: f32) -> f32 {
    let i = x.to_bits();
    let i = 0x5f3759df - (i >> 1);
    let y = f32::from_bits(i);

    y * (1.5 - 0.5 * x * y * y)
}


#[derive(Copy, Clone)]
pub struct Range {
    pub smaller_corner: V3,
    pub greater_corner: V3,
}

impl Range {
    pub fn new(smaller_corner: V3, greater_corner: V3) -> Self {
        Range {smaller_corner, greater_corner}
    }
    pub fn contain(&self, p: V3) -> bool {
        let x_good = self.smaller_corner.x < p.x && p.x < self.greater_corner.x;
        let y_good = self.smaller_corner.y < p.y && p.y < self.greater_corner.y;
        let z_good = self.smaller_corner.z < p.z && p.z < self.greater_corner.z;

        x_good && y_good && z_good
    }
    pub fn intersection(&self, other: &Self) -> Option<Self> {
        let a_s = self.smaller_corner;
        let a_g = self.greater_corner;
        let b_s = other.smaller_corner;
        let b_g = other.greater_corner;

        // new smaller corner
        let s_x = f32::max(a_s.x, b_s.x);
        let s_y = f32::max(a_s.y, b_s.y);
        let s_z = f32::max(a_s.z, b_s.z);

        // new greater corner
        let b_x = f32::max(a_g.x, b_g.x);
        let b_y = f32::max(a_g.y, b_g.y);
        let b_z = f32::max(a_g.z, b_g.z);

        if s_x < b_x && s_y < b_y && s_z < b_z {
            Some(
                Self::new(
                    V3::new(s_x, s_y, s_z),
                    V3::new(b_x, b_y, b_z),
                )
            )
        }
        else {
            None
        }
    }
    pub fn diagonal(&self) -> V3 {
        self.greater_corner - self.smaller_corner
    }
}



pub trait Dist {
    // signed distance function
    fn dist(&self, point: V3) -> f32;
}

impl<F> Dist for F where F: Fn(V3) -> f32 {
    fn dist(&self, point: V3) -> f32 {
        self(point)
    }
}
