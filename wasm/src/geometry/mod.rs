mod vec_3d;
use vec_3d::V3;

mod noise;
mod random;
//mod octree;

// a vertex currently has 12 values: x, y, z  |  r, g, b and so on
const SIZE_VERTEX : usize = 12;


fn get_point(points: &Vec<f32>, i: u16) -> V3 {
    let i = i as usize*SIZE_VERTEX;
    V3::new(
        points[i],
        points[i+1],
        points[i+2])
}


macro_rules! push_index {
    // ex. push_index!(indices, [9, 3, 2, 42])
    ($array: ident, [$($x: expr),*]) => {
        $(
            $array.push($x as u16);
        )*
    };
    // ex. push_index!(indices, tuple.[0, 2, 1, 2, 0, 1])
    ($array: ident, $tuple: ident.[$($x: tt),*]) => {
        $(
            $array.push($tuple.$x as u16);
        )*
    };
}

macro_rules! push_point {
    ($array:ident) => {};
    ($array:ident, [$a:expr, $b:expr, $c:expr] $(,$tail:tt)*) => {
        $array.push($a as f32);
        $array.push($b as f32);
        $array.push($c as f32);
        push_point!($array $(,$tail)*)
    };
    ($array:ident, $v:expr $(,$tail:tt)*) => {
        $array.push($v.x as f32);
        $array.push($v.y as f32);
        $array.push($v.z as f32);
        push_point!($array $(,$tail)*)
    };
}

fn pseudo_sphere(points: &mut Vec<f32>, indices: &mut Vec<u16>, center: V3, radius: f32, color: (f32, f32, f32)) {
    let frequency = 0.3+random::rand_float();
    let i0 = points.len()/SIZE_VERTEX;
    let n = 30usize;
    let pi = 3.15;

    let range = (V3::new(-2.1, -2.1, -2.1), V3::new(2.1, 2.1, 2.1));
    let shape_noise = noise::Perlin::new(range, (4, 4, 4), 2.4);
    let phase_noise = noise::Perlin::new(range, (8, 8, 8), 1.5);

    // create points
    for long in 0..n {
        for lat in 0..n {
            let a1 = lat as f32 / (n as f32) * 2.0 * pi;
            let a2 = long as f32 / (n as f32 -1.0) * pi;

            let p = V3::new(
                a1.cos()*a2.sin(), 
                a1.sin()*a2.sin(),
                a2.cos()         ,
            );

            let rad_vector = p.scale(radius+shape_noise.noise(p));

            let v = p.scale(0.2)+random::rand_v3().scale(0.3);

            push_point!(points,
                (center+rad_vector), // coordinates
                (V3::from(color)), // color
                v, // 
                [frequency, phase_noise.noise(p), 0.0]
            );
        }
    }

    // create triangles
    for long in 0..n-1 {
        for lat in 0..n-1 {
            let i = i0 + long*n+lat;
            let p = (i, i+1, i+n, i+n+1);
            push_index!(indices, p.[0, 3, 1, 0, 2, 3]);
        }
    }
    for long in 0..n-2 {
        let i = i0 + long*n;
        let p = (i+n-1, i+n, i+n+n-1, i+n+n);
            push_index!(indices, p.[0, 3, 1,   0, 2, 3]);
    }
}

pub fn test_sphere(points: &mut Vec<f32>, indices: &mut Vec<u16>) {
    use random::rand_float;
    for _ in 0..30 {
        let v = random::rand_v3().scale(20.0+rand_float()*40.0);
        let center = V3::new(v.x, v.y, 2.0+rand_float()*8.0);
        let color = (rand_float(), rand_float(), rand_float());
        pseudo_sphere(points, indices, center, 0.5+rand_float(), color);
    }
}



pub fn rand_surface(points: &mut Vec<f32>, indices: &mut Vec<u16>) {
    // we generate fractal noise with 2d slices of 3d perlin noise
    let range = (V3::new(-100.0, -100.0, -1.0), V3::new(100.0, 100.0, 1.0));
    let perlin_1 = noise::Perlin::new(range, (5, 5, 3), 15.0);
    let perlin_2 = noise::Perlin::new(range, (30, 30, 3), 5.5);

    let phase_noise = noise::Perlin::new(range, (30, 30, 3), 6.28);

    let i0 = points.len()/SIZE_VERTEX;
    let n = 100usize;
    for x in 0..n {
        for y in 0..n {
            let x = x as f32-50.0+0.5; 
            let y = y as f32-50.0;
            let v = V3::new(x, y, 0.0);
            let z = perlin_1.noise(v)+perlin_2.noise(v)-4.0;
            // position
            push_point!(
                points, 
                [x, y, z],
                [0.7, 0.4, 0.3],
                [0.0, 0.0, 0.3],
                [1.0, phase_noise.noise(v), 0.0]);
        }
    }

    for x in 0..n-1 {
        for y in 0..n-1 {
            let i = i0 + y*n+x;
            // corners of square
            let p = (i, i+1, i+n, i+n+1);
            // 3 triangles
            push_index!(indices, p.[0, 3, 1,  0, 2, 3]);
        }
    }
}




// shading algorithm
pub fn shade(points: &mut Vec<f32>, indices: &Vec<u16>) {
    let light_dir : V3 = V3::new(0.3, 0.3, 0.3);

    // this vector will store the average normal of each point
    let mut normals_by_point = vec![V3::null(); points.len()/SIZE_VERTEX];

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
        points[i*SIZE_VERTEX+3] += brightness;
        points[i*SIZE_VERTEX+4] += brightness;
        points[i*SIZE_VERTEX+5] += brightness;
    }
}

