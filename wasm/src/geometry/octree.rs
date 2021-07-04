use std::ops::Index;

use super::V3;


pub trait Dist {
    // signed distance function
    fn dist(&self, point: V3) -> f32;
}

// bool structure: intersection, union and negation
trait BoolLike {
    fn union(a: Self, b: Self) -> Self;
    fn inter(a: Self, b: Self) -> Self;
    fn not(self) -> Self;
}


#[derive(PartialEq)]
struct Couple<T>(T, T);


impl<T> Index<bool> for Couple<T> {
    type Output = T;

    fn index(&self, b: bool) -> &Self::Output {
        if b {&self.0} else {&self.1}
    }
}


//                  _                              _   _             
// __ ___ _  _ _ __| |___   ___ _ __  ___ _ _ __ _| |_(_)___ _ _  ___
/// _/ _ \ || | '_ \ / -_) / _ \ '_ \/ -_) '_/ _` |  _| / _ \ ' \(_-<
//\__\___/\_,_| .__/_\___| \___/ .__/\___|_| \__,_|\__|_\___/_||_/__/
//            |_|              |_|                                   


impl<T> Couple<T> {
    // TODO: rename
    fn new(mut f: impl FnMut(bool) -> T) -> Self {
        Self(f(false), f(true))
    }
    // execute a function on each item
    fn each(&self, mut f: impl FnMut(&T, bool)) {
        f(&self.0, false); f(&self.1, true)
    }
    // a map implementation on the 2 items
    fn map<S>(&self, f:impl Fn(&T) -> S) -> Couple<S> {
        Couple (f(&self.0), f(&self.1))
    }
    fn fusion(&self, f:impl Fn(&T, &T) -> T) -> T {
        f(&self.0, &self.1)
    }
}

impl<T> Couple<Couple<T>> {
    fn map2<S>(&self, f:impl Fn(&T) -> S) -> Couple<Couple<S>> {
        Couple (self.0.map(&f), self.1.map(&f))
    }
    fn fusion2(&self, f:impl Fn(&T, &T) -> T) -> T {
        f(&self.0.fusion(&f), &self.1.fusion(&f))
    }
}

impl<T> Couple<Couple<Couple<T>>> {
    fn map3<S>(&self, f:impl Fn(&T) -> S) -> Couple<Couple<Couple<S>>> {
        Couple (self.0.map2(&f), self.1.map2(&f))
    }
    fn fusion3(&self, f:impl Fn(&T, &T) -> T) -> T {
        f(&self.0.fusion2(&f), &self.1.fusion2(&f))
    }
}

pub fn sub_corner(half_size: V3, bx: bool, by: bool, bz: bool) -> V3 {
    V3::new(
        Couple(0.0, half_size.x)[bx],
        Couple(0.0, half_size.y)[by],
        Couple(0.0, half_size.z)[bz]
    )
}

impl<T> BoolLike for Couple<T> where T: BoolLike {
    fn union(a: Self, b: Self) -> Self {
        Couple (
            T::union(a.0, b.0),
            T::union(a.1, b.1),
        )
    }
    fn inter(a: Self, b: Self) -> Self {
        Couple (
            T::inter(a.0, b.0),
            T::inter(a.1, b.1),
        )
    }
    fn not(self) -> Self {
        Couple ( self.0.not(), self.1.not() )
    }
}

//              _     
// _ _  ___  __| |___ 
//| ' \/ _ \/ _` / -_)
//|_||_\___/\__,_\___|
                    
type Subcubes = Box<Couple<Couple<Couple<Node>>>>;

pub enum Node {
    Sub(Subcubes), // subcubes
    Point(V3), // point
    Empty, // completely outside
    Full, // completely inside
}

impl Node {
    fn approximate<S: Dist>(corner: V3, half_size: V3, shape: &S, depth: usize) -> Node {
        // approximate distance function with octree
        let center = corner + half_size;
        let dist = shape.dist(center);

        if depth == 0 {
            Node::Point(center)
        }
        else {
            let diag = half_size.norm();
            if dist > diag { // if distance is 
                Node::Empty
            }
            else if dist < -diag {
                Node::Full
            }
            else {
                let cubes = Couple::new(|bx| Couple::new(|by| Couple::new(|bz| 
                            Node::approximate(
                                corner + sub_corner(half_size, bx, by, bz),
                                half_size.scale(0.5), shape, depth-1))));

                Node::Sub(Box::new(cubes))
            }
        }
    }


    fn find_points_in(&self, corner: V3, half_size: V3, min_p: V3, max_p: V3, points: &mut Vec<V3>) {
        // find all the points in this zone in the octree
        let corner2 = corner + half_size.scale(0.5);
        let x_range = corner.x < max_p.x && min_p.x < corner2.x;   
        let y_range = corner.y < max_p.y && min_p.y < corner2.y;   
        let z_range = corner.z < max_p.z && min_p.z < corner2.z;   

        if x_range && y_range && z_range {
            match self {
                Node::Sub(cubes) => 
                    cubes.each(|x, bx|
                        x.each(|y, by|
                            y.each(|node, bz|
                                node.find_points_in(corner+sub_corner(half_size, bx, by, bz), half_size.scale(0.5), min_p, max_p, points)
                            )
                        )
                    ),
                Node::Point(p) => points.push(*p),
                _ => ()
            }
        }
    }

    fn get_points(&self, points: &mut Vec<V3>) {
        match self {
            Node::Sub(cubes) => 
                cubes.each(|x, _|
                    x.each(|y, _|
                        y.each(|node, _| 
                            node.get_points(points)
                        )
                    )
                ),

            Node::Point(p) => {
                points.push(*p);
            },
            _ => ()
        }
    }
}

impl BoolLike for Node {
    fn union(a: Self, b: Self) -> Self {
        // union of 2 octree nodes at the same location
        match (a, b) {
            (Node::Sub(cubes_a), Node::Sub(cubes_b)) => {
                // calculate the union of the sub_cubes
                let cubes = BoolLike::union(*cubes_a, *cubes_b);

                let fusion = cubes
                    // get a single State representing if they are 
                    // all `empty` or all `full`
                    .map3(|node| Some(node)) // TODO: fix
                    .fusion3(
                        |a, b|  match (a, b) {
                            (Some(&Node::Empty), Some(&Node::Empty)) => Some(&Node::Empty),
                            (Some(&Node::Full), Some(&Node::Full)) => Some(&Node::Empty),
                             _ => None,
                    });

                // if the new contain only empty or full blocks, return one of them
                match fusion {
                    Some(Node::Empty) => Node::Full,
                    Some(Node::Full) => Node::Empty,
                    _ => Node::Sub(Box::new(cubes))
                }
            },
            // if one is empty, return the other
            (Node::Empty, b) => b,
            (a, Node::Empty) => a,
            // if one is full, return full
            (Node::Full, _) => Node::Full,
            (_, Node::Full) => Node::Full,
            // if one is point, return it
            (Node::Point(p), _) => Node::Point(p),
            (_, Node::Point(p)) => Node::Point(p),
        }
    }
    fn inter(a: Self, b: Self) -> Self {
        // union of 2 octree nodes at the same location
        match (a, b) {
            (Node::Sub(cubes_a), Node::Sub(cubes_b)) => {
                // calculate the union of the sub_cubes
                let cubes = BoolLike::inter(*cubes_a, *cubes_b);

                let fusion = cubes
                    // get a single State representing if they are 
                    // all `empty` or all `full`
                    .map3(|node| Some(node)) // TODO: fix
                    .fusion3(
                        |a, b| match (a, b) {
                            (Some(&Node::Empty), Some(&Node::Empty)) => Some(&Node::Empty),
                            (Some(&Node::Full), Some(&Node::Full)) => Some(&Node::Empty),
                             _ => None,
                    });

                // if the new contain only empty or full blocks, return one of them
                match fusion {
                    Some(Node::Empty) => Node::Full,
                    Some(Node::Full) => Node::Empty,
                    _ => Node::Sub(Box::new(cubes))
                }
            },
            // if one is full, return the other
            (Node::Full, b) => b,
            (a, Node::Full) => a,
            // if one is empty, return empty
            (Node::Empty, _) => Node::Empty,
            (_, Node::Empty) => Node::Empty,
            // if one is point, return it
            (Node::Point(p), _) => Node::Point(p),
            (_, Node::Point(p)) => Node::Point(p),
        }
    }
    fn not(self) -> Self {
        match self {
            Node::Sub(cubes) => Node::Sub(Box::new((*cubes).not())),
            Node::Point(id) => Node::Point(id),
            Node::Empty => Node::Full,
            Node::Full => Node::Empty,
        }
    }
}

pub struct Octree {
    dim: V3,
    depth: usize,
    root: Node
}


impl Octree {
    // TODO: constructor with distance function and range
    //       define what is a range in v3
    //pub fn triangulate(&mut self, point_array: Float32Array, triangles: Uint16Array
}
