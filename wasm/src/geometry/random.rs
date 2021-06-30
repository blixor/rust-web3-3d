use getrandom;
use super::V3;


pub fn rand_float() -> f32 {
   const SCALE_FACTOR: f32 = 1.0/65536.0;

   let mut buff = [0; 2];
   getrandom::getrandom(&mut buff).unwrap();
   let (a, b) = (buff[0] as f32, buff[1] as f32);
   (a*256.0+b) * SCALE_FACTOR
}

pub fn rand_v3() -> V3 {
    const SCALE_FACTOR: f32 = 2.0/65536.0;
    let mut buff = [0; 6];
    getrandom::getrandom(&mut buff).unwrap();
    let x = buff[0] as f32 * 256.0 + buff[1] as f32;
    let y = buff[2] as f32 * 256.0 + buff[3] as f32;
    let z = buff[4] as f32 * 256.0 + buff[5] as f32;

    (V3{x, y, z}.scale(SCALE_FACTOR)-V3::new(1.0, 1.0, 1.0)).normalize()
}
