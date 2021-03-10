use wasm_bindgen::prelude::*;

use web_sys::WebGlRenderingContext as GL;
use web_sys::WebGlUniformLocation;

mod webgl;
use webgl::Engine;

mod camera;
use camera::Camera;

mod geometry;

#[wasm_bindgen]
pub struct Universe {
    engine: Engine,
    camera: Camera,
    n_update: u32,
    t_update: u32,
}
 
#[wasm_bindgen]
impl Universe {
    #[wasm_bindgen(constructor)]
    pub fn new(gl: GL, trans_location: WebGlUniformLocation, t: u32) -> Self {
        let camera = Camera {x:-3.0, y:0.0, z:0.0, angle:0.20};
        let engine = Engine {gl, trans_location, n_indices: 0i32};
        Self {engine, camera, n_update: 0, t_update: t}
    }

    pub fn update(&mut self, t: u32, left: bool, right: bool, down: bool, up: bool) {

        let dt = (t - self.t_update) as f32 / 1000.0;



        if left  {self.camera.rotate( 2.0 * dt);};
        if right {self.camera.rotate( 2.0 * -dt);};
        if down  {self.camera.forward(2.0 * -dt);};
        if up    {self.camera.forward(2.0 * dt);};

        if self.t_update % 5 == 0 {
            // update landscape
            let (test_points, test_index) = geometry::test_sphere(1.0, true);
            self.engine.update_triangles(test_points, test_index);
        }

        self.n_update += 1;
        self.t_update = t;

    }

    pub fn render(&mut self){
        self.engine.render(self.camera.get_transform(self.engine.width(), self.engine.height()));
    }

    pub fn rotate(&mut self, d_angle: f32) {self.camera.rotate(d_angle)}
    pub fn forward(&mut self, d: f32) {self.camera.forward(d)}
    pub fn angle(&self) -> f32 {self.camera.angle}
}
