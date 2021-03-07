pub fn test_cube(scale: f32) -> (Vec<f32>, Vec<u16>) {
    let a = scale;
    let mut points = Vec::with_capacity(52);
    let indeces = 
        [
            0, 5, 4, 0, 1, 5,
            1, 3, 7, 1, 7, 5,
            3, 2, 6, 3, 6, 7,
            2, 0, 4, 2, 4, 6,
            4, 7, 5, 4, 6, 7,
            0, 3, 1, 0, 2, 3,
        ].to_vec();

    for &z in &[-a, a] {
        for &y in &[-a, a] {
            for &x in &[-a, a] {
                points.push(x);
                points.push(y);
                points.push(z);
                points.push(x);
                points.push(0.5);
                points.push(0.5);
            }
        }
    }
    (points, indeces)
}
/*
pub fn demo(gl: GL, shape: Catalog, resol: u32) -> u32 {

    let n_points = match shape {
        Catalog::Cube => 8,
        _ => resol*resol,
    };

    let n_triangles =  match shape {
        Catalog::Cube => 12,
        Catalog::Sphere => resol*resol,
        Torus => resol*resol*2,
    };

    let size = resol as usize;

    // 3 coordonées et (r g b) par point
    let point_array = Float32Array::new_with_length(n_points*6);

    // 3 points par triangle
    let index_array = Uint16Array::new_with_length(n_triangles*3);


    match shape {
        Catalog::Cube => {
                        /**************
                        * // LE CUBE *
                        **************/
            let mut i: u32 = 0;
            let a = 1f32;
            for z in vec![-a, a] {
                for y in vec![-a, a] {
                    for x in vec![-a, a] {
                        // position
                        point_array.set_index(i, x);
                        point_array.set_index(i+1, y);
                        point_array.set_index(i+2, z);

                        // couleur
                        point_array.set_index(i+3, 0.5+x/5.0);
                        point_array.set_index(i+4, 0.5+y/5.0);
                        point_array.set_index(i+5, 0.5+z/5.0);
                        i += 6;
                    }
                }
            }

            let indeces : [u16; 36] = 
                [
                    0, 5, 4, 0, 1, 5,
                    1, 3, 7, 1, 7, 5,
                    3, 2, 6, 3, 6, 7,
                    2, 0, 4, 2, 4, 6,
                    4, 7, 5, 4, 6, 7,
                    0, 3, 1, 0, 2, 3,
                ];
                
            for (i, p) in indeces.iter().enumerate() {
                index_array.set_index(i as u32, *p);
            }
        },
        Catalog::Sphere => {
                    /****************
                    * // LA SPHÈRE *
                    ****************/

            let mut i = 0u32;
            for lat in 0..size {
                for long in 0..size {
                    let a1 = (long as f32)/(size as f32)*6.28;
                    let a2 = (lat as f32)/(size as f32)*3.14;

                    point_array.set_index(i, a1.cos()*a2.sin());
                    point_array.set_index(i+2, a1.sin()*a2.sin());
                    point_array.set_index(i+1, a2.cos());

                    point_array.set_index(i+3, 0.5+a1.cos()/2.0);
                    point_array.set_index(i+5, 0.5+a2.sin()/2.0);
                    point_array.set_index(i+4, 0.5+a2.sin()/2.0);
                    i += 6;
                }
            }
            i = 0;
            for lat in 0..size {
                for long in 0..(size-1) {
                    let p = (long*size + lat, long*size + (lat+1)%size, (long+1)*size + lat, (long+1)*size + (lat+1)%size);
                    index_array.set_index((i  ) as u32, p.0 as u16);
                    index_array.set_index((i+1) as u32, p.2 as u16);
                    index_array.set_index((i+2) as u32, p.1 as u16);

                    i += 3;
                }
            }
        },
        Catalog::Torus => {
                    /**************
                    * // LE TORE *
                    **************/
            let mut i = 0u32;
            for lat in 0..size {
                for long in 0..size {
                    let a1 = (lat as f32)/(size as f32-1.0)*6.29;
                    let a2 = (long as f32)/(size as f32-1.0)*6.29;
    
                    point_array.set_index(i, 0.5*a1.cos()*(a2.sin()+2.0));
                    point_array.set_index(i+1, 0.5*a1.sin()*(a2.sin()+2.0));
                    point_array.set_index(i+2, 0.5*a2.cos());
    
                    point_array.set_index(i+3, 0.5+a1.cos()/2.0);
                    point_array.set_index(i+4, 0.5+a2.sin()/2.0);
                    point_array.set_index(i+5, 0.5+a2.sin()/2.0);
                    i += 6;
                }
            }
            i = 0;
    
            for lat in 0..size {
                for long in 0..(size-1) {
                    let p = (long*size + lat, long*size + (lat+1)%size, (long+1)*size + lat, (long+1)*size + (lat+1)%size);
                    index_array.set_index((i  ) as u32, p.0 as u16);
                    index_array.set_index((i+1) as u32, p.1 as u16);
                    index_array.set_index((i+2) as u32, p.2 as u16);
    
                    index_array.set_index((i+3) as u32, p.1 as u16);
                    index_array.set_index((i+4) as u32, p.3 as u16);
                    index_array.set_index((i+5) as u32, p.2 as u16);
    
                    i += 6;
                }
            }
        }
    }


    gl. buffer_data_with_array_buffer_view(
        GL::ARRAY_BUFFER, 
        &point_array, 
        GL::DYNAMIC_DRAW
    );

    gl. buffer_data_with_array_buffer_view(
        GL::ELEMENT_ARRAY_BUFFER, 
        &index_array, 
        GL::DYNAMIC_DRAW
    );

    n_triangles
}
*/
