type M4 = [f32; 16];


pub fn mult(a: M4, b: M4) -> M4 {
    [
        b[0] *a[0]  +  b[1] *a[4]  +  b[2] *a[8]   +  b[3] *a[12],
        b[0] *a[1]  +  b[1] *a[5]  +  b[2] *a[9]   +  b[3] *a[13],
        b[0] *a[2]  +  b[1] *a[6]  +  b[2] *a[10]  +  b[3] *a[14],
        b[0] *a[3]  +  b[1] *a[7]  +  b[2] *a[11]  +  b[3] *a[15],
        b[4] *a[0]  +  b[5] *a[4]  +  b[6] *a[8]   +  b[7] *a[12],
        b[4] *a[1]  +  b[5] *a[5]  +  b[6] *a[9]   +  b[7] *a[13],
        b[4] *a[2]  +  b[5] *a[6]  +  b[6] *a[10]  +  b[7] *a[14],
        b[4] *a[3]  +  b[5] *a[7]  +  b[6] *a[11]  +  b[7] *a[15],
        b[8] *a[0]  +  b[9] *a[4]  +  b[10]*a[8]   +  b[11]*a[12],
        b[8] *a[1]  +  b[9] *a[5]  +  b[10]*a[9]   +  b[11]*a[13],
        b[8] *a[2]  +  b[9] *a[6]  +  b[10]*a[10]  +  b[11]*a[14],
        b[8] *a[3]  +  b[9] *a[7]  +  b[10]*a[11]  +  b[11]*a[15],
        b[12]*a[0]  +  b[13]*a[4]  +  b[14]*a[8]   +  b[15]*a[12],
        b[12]*a[1]  +  b[13]*a[5]  +  b[14]*a[9]   +  b[15]*a[13],
        b[12]*a[2]  +  b[13]*a[6]  +  b[14]*a[10]  +  b[15]*a[14],
        b[12]*a[3]  +  b[13]*a[7]  +  b[14]*a[11]  +  b[15]*a[15]
    ]
}



pub fn projection(a: f32, fov: f32, z_near: f32, z_far: f32) -> M4 {

    /*
     * a is aspect ratio
     * fov is the field of view (not in radian, in proportion)
     * z_near is the distance to the nearest plane of the frustrum
     * z_far is the distance to the furthest plane of the frustrum
     */

    let r = 1.0 / (z_near - z_far);

    [
      fov/a, 0.0,  0.0,                  0.0,
      0.0,   fov,  0.0,                  0.0,
      0.0,   0.0,  (z_near + z_far)*r,  -1.0,
      0.0,   0.0,  2.0*z_near*z_far*r,   0.0
    ]
}
