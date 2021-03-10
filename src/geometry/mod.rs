mod vec_3d;
use vec_3d::V3;


fn get_point(points: &Vec<f32>, i: u16) -> V3 {
    let i = (i*6) as usize;
    V3::new(
        points[i],
        points[i+1],
        points[i+2])
}

pub fn test_sphere(scale: f32, shading_enabled: bool) -> (Vec<f32>, Vec<u16>) {
    let n = 100usize;
    let mut points = Vec::with_capacity(n*n*6usize);
    let mut indices = Vec::<u16>::new();
    let pi = 3.0;

    // create points
    for long in 0..n {
        for lat in 0..n {
            let a1 = lat as f32 / (n as f32 - 1.0) * 2.0 * pi;
            let a2 = long as f32 / (n as f32 - 1.0) * pi;

            points.push(a1.cos()*a2.sin()*scale); // x
            points.push(a1.sin()*a2.sin()*scale); // y
            points.push(a2.cos()         *scale); // z

            points.push(0.3); // r
            points.push(0.5); // g
            points.push(0.7); // b
        }
    }

    // create triangles
    for long in 0..n-1 {
        for lat in 0..n {
            let p = (long*n + lat, long*n + (lat+1)%n, (long+1)*n + lat, (long+1)*n + (lat + 1)%n);
            indices.push(p.0 as u16);
            indices.push(p.1 as u16);
            indices.push(p.3 as u16);

            indices.push(p.0 as u16);
            indices.push(p.3 as u16);
            indices.push(p.2 as u16);

        }
    }

    if shading_enabled {
        shade(&mut points, &indices, V3::new(0.4, 0.4, 0.4));
    }
    (points, indices)
}



fn shade(points: &mut Vec<f32>, indices: &Vec<u16>, light_dir: V3) {
    // this vector will store the average normal of each point
    let mut normals_by_point = vec![V3::null(); points.len()/6];

    for i in (0..indices.len()).step_by(3) {
        let p1 = get_point(points, indices[i  ]);
        let p2 = get_point(points, indices[i+1]);
        let p3 = get_point(points, indices[i+2]);

        // compute normal of each triangle
        let normal = V3::cross(p1-p3, p1-p2).normalize();

        // then add this vector to the 3 points making up this triangle
        normals_by_point[indices[i  ] as usize] += normal;
        normals_by_point[indices[i+1] as usize] += normal;
        normals_by_point[indices[i+2] as usize] += normal;
    }

    for (i, &v) in normals_by_point.iter().enumerate() {
        let brightness = V3::dot(v.normalize(), light_dir); // ⚠ normalize
        points[i*6+3] += brightness;
        points[i*6+4] += brightness;
        points[i*6+5] += brightness;
    }
}


//pub fn test_plane() -> (Vec<f32>, Vec<u16>) {
//    let mut points = Vec::new();
//    let mut indices = Vec::new();
//    let n = 30usize;
//    for ix in 0..n {
//        for iy in 0..n {
//            let x = ix as f32 / (n as f32);
//            let y = iy as f32 / (n as f32);
//            points.push(x);
//            points.push(y);
//            points.push(0.0);
//
//            points.push(0.5);
//            points.push(0.5);
//            points.push(0.5);
//        }
//    }
//    for ix in 0..n-1 {
//        for iy in 0..n-1 {
//            let p = (iy*n+ix, iy*n+ix+1, (iy+1)*n+ix, (iy+1)*n+ix+1);
//            indices.push(p.0 as u16);
//            indices.push(p.1 as u16);
//            indices.push(p.2 as u16);
//
//            indices.push(p.1 as u16);
//            indices.push(p.3 as u16);
//            indices.push(p.2 as u16);
//        }
//    }
//    shade(&mut points, &indices, V3::new(0.3, 0.3, 0.3));
//    (points, indices)
//}
