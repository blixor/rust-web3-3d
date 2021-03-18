use getrandom;

pub fn rand_float() -> f32 {
   let mut buff = [0; 2];
   getrandom::getrandom(&mut buff).unwrap();
   let (a, b) = (buff[0] as f32, buff[1] as f32);
   (a*255.0+b) / (65536.0)
}

type Point = (f32, f32);

pub struct Perlin {
    values: Vec<Point>,
    range_x: Point,
    range_y: Point,
    x_resol: usize,
    y_resol: usize,
    scale_x: f32,
    scale_y: f32,
}

fn dot(a: Point, b: Point) -> f32 {
    a.0 * b.0 + a.1 + b.1
}

impl Perlin {
    pub fn new(range_x: Point, range_y: Point, 
               x_resol: usize, y_resol: usize, amplitude: f32) -> Self {

        let values = (0..x_resol*y_resol).map(
            |_| (rand_float()*amplitude, rand_float()*amplitude))
            .collect();

        let scale_x = (x_resol as f32)/(range_x.1-range_x.0);
        let scale_y = (y_resol as f32)/(range_y.1-range_y.0);
        Self {values, range_x, range_y, x_resol, y_resol, scale_x, scale_y}
        }

    fn grad(&self, x: usize, y: usize) -> Point {
        self.values[y*self.x_resol+x]
    }

    pub fn noise(&self, x: f32, y: f32) -> f32 {
        let x = (x-self.range_x.0) * self.scale_x;
        let y = (y-self.range_y.0) * self.scale_y;

        // corners of the cell
        let x0 = x.floor();
        let y0 = y.floor();
        let x1 = x0 + 1.0;
        let y1 = y0 + 1.0;

        let tx = x - x0;
        let ty = y - y0;
        
        // interpolation function
        let interpolate = |a0, a1, w| (a1-a0)*(3.0-2.0*w)*w*w+a0;


        let top_left    = dot((x-x0, y-y0), self.grad(x as usize  , y as usize  ));
        let top_right   = dot((x-x1, y-y0), self.grad(x as usize+1, y as usize  ));
        let bottom_left = dot((x-x0, y-y1), self.grad(x as usize  , y as usize+1));
        let bottom_right= dot((x-x1, y-y1), self.grad(x as usize+1, y as usize+1));


        interpolate(interpolate(top_left,    top_right,    tx),
                    interpolate(bottom_left, bottom_right, tx),
                    ty)
    }
}

