mod matrix;


pub struct Camera {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub angle: f32,
}


impl Camera {
    pub fn rotate(&mut self, d_angle: f32){
        self.angle += d_angle
    }

    pub fn forward(&mut self, d: f32){
        self.x += self.angle.cos()*d;
        self.y += self.angle.sin()*d;
    }

    pub fn get_transform(&self, width: u32, height: u32) -> [f32; 16] {
        let a = (width as f32) / (height as f32);

        let cos = (-self.angle).cos();
        let sin = (-self.angle).sin();
        
        // inverse matrix of the camera (rotation of angle + translation)
        let inv_cam = [
            cos,    0.0,    -sin,      0.0,
            0.0,    1.0,    0.0,       0.0,
            sin,    0.0,    cos,       0.0,
            self.x*sin+self.y*cos, -self.z, self.x*cos-self.y*sin, 1.0
        ];

        matrix::mult(matrix::projection(a, 0.01, 500.0), inv_cam)
    }
}
