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
    pub fn up(&mut self, d: f32) {
        self.z += d;
    }
    pub fn get_info(&self) -> String {
        format!("x = {}, y = {}, z = {}", self.x, self.y, self.z)
    }

    pub fn get_transform(&self, width: u32, height: u32) -> [f32; 16] {
        let a = (width as f32) / (height as f32);

        let cos = (-self.angle).cos();
        let sin = (-self.angle).sin();
        

        let inv_cam = [
            -sin, 0.0, -cos, 0.0,
            -cos, 0.0,  sin, 0.0,
            0.0,  1.0,  0.0, 0.0,

            // last column:
            self.x*sin+self.y*cos,
            -self.z,
            self.x*cos-self.y*sin,
            1.0,
        ];
        /* to generate this matrix, we use these 2 transformations:

        let translation = [ // move the scene to the position of the camera
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            -self.x, -self.y, -self.z, 1.0,
        ];

        let rot = [ // rotate with angle and transpose
            -sin, 0.0,-cos, 0.0,
            -cos, 0.0, sin, 0.0,
             0.0, 1.0, 0.0, 0.0,
             0.0, 0.0, 0.0, 1.0,
        ];

        let inv_cam = matrix::mult(rot, translation);
        */
        
        matrix::mult(
            matrix::projection(a, 1.5, 0.1, 100.0),
            inv_cam)
    }
}
