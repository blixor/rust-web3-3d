use web_sys::WebGlRenderingContext as GL;
use web_sys::WebGlUniformLocation;

use js_sys::*;

pub struct Engine {
    pub gl: GL,
    pub trans_location: WebGlUniformLocation,
    pub n_indices: i32, 
}

impl Engine {
    pub fn update_triangles(&mut self, point_data: Vec<f32>, index_data: Vec<u16>) {
        unsafe {
            let vert_array = Float32Array::view(&point_data[..]);
            let index_array = Uint16Array::view(&index_data[..]);

            self.gl.buffer_data_with_array_buffer_view(
                GL::ARRAY_BUFFER,
                &vert_array,
                GL::DYNAMIC_DRAW);

            self.gl.buffer_data_with_array_buffer_view(
                GL::ELEMENT_ARRAY_BUFFER,
                &index_array,
                GL::DYNAMIC_DRAW);
        }

        self.n_indices = index_data.len() as i32;
    }

    pub fn render(&self, transform: [f32; 16] ) {
        self.gl.uniform_matrix4fv_with_f32_array(
            Some(&self.trans_location),
            false,
            &transform,
        );

        self.gl.clear(GL::COLOR_BUFFER_BIT);
        self.gl.draw_elements_with_i32(GL::LINES, self.n_indices, GL::UNSIGNED_SHORT, 0)
    }
    pub fn width(&self) -> u32 {self.gl.drawing_buffer_width() as u32}
    pub fn height(&self) -> u32 {self.gl.drawing_buffer_height() as u32}
}

