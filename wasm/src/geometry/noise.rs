use super::random;
use super::V3;
use super::Range;

pub struct Perlin {
    values: Vec<V3>,
    range: Range,
    resol: (usize, usize, usize),
    scale_x: f32,
    scale_y: f32,
    scale_z: f32,
    amplitude: f32,
}


impl Perlin {
    pub fn new(range: Range, resol: (usize, usize, usize), amplitude: f32) -> Self {
        let values = (0..resol.0*resol.1*resol.2)
            .map(|_| random::rand_v3())
            .collect();

        let diag = range.diagonal();
        let scale_x = (resol.0 as f32 - 1.0)/diag.x;
        let scale_y = (resol.1 as f32 - 1.0)/diag.y;
        let scale_z = (resol.2 as f32 - 1.0)/diag.z;

        Self {values, range, resol, scale_x, scale_y, scale_z, amplitude}
    }

    fn grad(&self, x: usize, y: usize, z: usize) -> V3 {
        self.values[
            z*self.resol.0*self.resol.1
            +y*self.resol.0
            +x]
    }

    pub fn noise(&self, v: V3) -> f32 {
        // map point to grid space
        let v = V3::new(
            (v.x - self.range.smaller_corner.x)*self.scale_x,
            (v.y - self.range.smaller_corner.y)*self.scale_y,
            (v.z - self.range.smaller_corner.z)*self.scale_z,
        );

        // first corner of the cell
        let c0 = V3::new(
            v.x.floor(),
            v.y.floor(),
            v.z.floor(),
        );

        // second corner of the cell
        let c1 = V3::new(
            c0.x+1.0,
            c0.y+1.0,
            c0.z+1.0,
        );

        let ux0 = c0.x as usize; let ux1 = c1.x as usize;
        let uy0 = c0.y as usize; let uy1 = c1.y as usize;
        let uz0 = c0.z as usize; let uz1 = c1.z as usize;

        let t0 = v-c0; // vector between the point and the first corner
        let t1 = v-c1; // vector between the point and the other corner

        // interpolation function
        let interpolate = |a0, a1, w| (a1-a0)*(3.0-2.0*w)*w*w+a0;

        // calculate a value for each corner
        let c000 = V3::dot(V3::new(t0.x, t0.y, t0.z), self.grad(ux0, uy0, uz0));
        let c001 = V3::dot(V3::new(t0.x, t0.y, t1.z), self.grad(ux0, uy0, uz1));
        let c010 = V3::dot(V3::new(t0.x, t1.y, t0.z), self.grad(ux0, uy1, uz0));
        let c011 = V3::dot(V3::new(t0.x, t1.y, t1.z), self.grad(ux0, uy1, uz1));
        let c100 = V3::dot(V3::new(t1.x, t0.y, t0.z), self.grad(ux1, uy0, uz0));
        let c101 = V3::dot(V3::new(t1.x, t0.y, t1.z), self.grad(ux1, uy0, uz1));
        let c110 = V3::dot(V3::new(t1.x, t1.y, t0.z), self.grad(ux1, uy1, uz0));
        let c111 = V3::dot(V3::new(t1.x, t1.y, t1.z), self.grad(ux1, uy1, uz1));

        // and combine them
        interpolate(
            interpolate(
                interpolate(c000, c001, t0.z),
                interpolate(c010, c011, t0.z),
                t0.y),
            interpolate(
                interpolate(c100, c101, t0.z),
                interpolate(c110, c111, t0.z),
                t0.y),
            t0.x) * self.amplitude
    }
}

