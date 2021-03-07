use std::ops::{Index};

mod geometry;
use geometry::{V3, Dist, Sphere};

use js_sys::*;

// bool structure: intersection, union and negation
trait BoolLike {
  fn union(self, other: Self) -> Self;
  fn inter(self, other: Self) -> Self;
  fn not(self) -> Self;
}


struct Couple<T>(T, T);

impl<T> Index<bool> for Couple<T> {
  type Output = T;
  
  fn index(&self, b: bool) -> &Self::Output {
    if b {&self.0} else {&self.1}
  }
}


impl<T: BoolLike> BoolLike for Couple<T> where T: BoolLike {
  fn union(self, other: Self) -> Self {
    Couple ( self.0.union(other.0), self.1.union(other.1))
  }
  fn inter(self, other: Self) -> Self {
    Couple ( self.0.inter(other.0), self.1.union(other.1))
  }
  fn not(self) -> Self {
    Couple ( self.0.not(), self.1.not() )
  }
}


impl<T> Couple<T> {
  fn new<F>(mut f: F) -> Self where F: FnMut(bool) -> T {
    Self(f(false), f(true))
  }
  fn map_same<F, S>(&self, f: F) -> Option<S> where F: Fn(&T) -> Option<S>, S: PartialEq {
    let result = (f(&self.0), f(&self.1));
    if result.0 == result.1 {result.0} else {None}
  }
  fn each<F, S>(&self, mut f: F) -> S where F: FnMut(&T, bool) -> S {
    f(&self.0, false); f(&self.1, true)
  }
  fn each_mut<F, S>(&mut self, mut f: F) -> S where F: FnMut(&mut T, bool) -> S {
    f(&mut self.0, false); f(&mut self.1, true)
  }
}



pub fn sub_corner(half_size: V3, bx: bool, by: bool, bz: bool) -> V3 {
  V3(Couple(0.0, half_size.0)[bx], Couple(0.0, half_size.1)[by], Couple(0.0, half_size.2)[bz])
}

type Subcubes = Box<Couple<Couple<Couple<Node>>>>;

#[derive(PartialEq)]
enum State { Empty, Full }
type Leaf = Result<u16, State>;
  
pub struct Node(Result<Subcubes, Result<u16, State>>);

impl Node {
  fn get_state(&self) -> Option<State> {
    match &self.0 {
      Err(Err(State::Full)) => Some(State::Full),
      Err(Err(State::Empty)) => Some(State::Empty),
      _ => None
      }
  }

  fn approximate<S: Dist>(corner: V3, half_size: V3, shape: &S, depth: usize) -> Node {
    let center = corner + half_size;
    let dist = shape.dist(center);

    if depth == 0 {
      Self (Err(Ok(0))) 
    }
    else {
      let diag = half_size.norm(); 
      if dist > diag {
        Self (Err(Err(State::Empty)))
      }
      else if dist < -diag {
        Self (Err(Err(State::Full))) 
      }
      else {
        let cubes = Couple::new(|bx| Couple::new(|by| Couple::new(|bz| 
          Node::approximate(
            corner + sub_corner(half_size, bx, by, bz),
            half_size*0.5, shape, depth-1))));

        Self(Ok(Box::new(cubes))) 
      }
    }
  }


  fn find_id_in(&self, corner: V3, half_size: V3, min_p: V3, max_p: V3, points: &mut Vec<u16>) {
    let corner2 = corner + half_size*0.5;
    let x_range = corner.0 < max_p.0 && min_p.0 < corner2.0;   
    let y_range = corner.1 < max_p.1 && min_p.1 < corner2.1;   
    let z_range = corner.2 < max_p.2 && min_p.2 < corner2.2;   

    if x_range && y_range && z_range {
      match &self.0 {
        Ok(cubes) => 
          cubes.each(|X, bx|
            X.each(|Y, by|
              Y.each(|node, bz|
                node.find_id_in(corner+sub_corner(half_size, bx, by, bz), half_size*0.5, min_p, max_p, points)
              )
            )
          ),
        Err(Ok(id)) => points.push(*id),
        _ => ()
      }
    }
  }

  fn get_points_and_index(&mut self, corner: V3, half_size: V3, points: &mut Vec<V3>) {
    match &mut self.0 {
      Ok(cubes) => 
        cubes.each_mut(|X, bx|
          X.each_mut(|Y, by|
            Y.each_mut(|node, bz| 
              node.get_points_and_index(corner + sub_corner(half_size, bx, by, bz), half_size*0.5, points)
            )
          )
        ),
      
      Err(Ok(id)) => {
        self.0 = Err(Ok(points.len() as u16));
        points.push(corner+half_size);
      },
      _ => ()
    }
  }
   

  /*pub fn get_links(&self, octree: &Octree, corner: V3, half_size: V3, links: &mut Vec<u16> {
    if self.state.is_none() {
      if let Some(cubes) = &self.subs {
        for (&bx, &by, &bz) in iproduct!(FT, FT, FT) {
          cubes[bx][by][bz].triangulate(octree, corner + sub_corner(half_size, bx, by, bz), half_size*0.5, links, index);
        }
      }
      else if let Some(my_id) = self.id {
        let center = corner+half_size;
        let size = half_size*2.0;

        let mut neighbourgs = [center; 6];
        neighbourgs[0].0 += size.0;
        neighbourgs[1].0 -= size.0;
        neighbourgs[2].1 += size.1;
        neighbourgs[3].1 -= size.1;
        neighbourgs[4].2 += size.2;
        neighbourgs[5].2 -= size.2;

        for id in neighbourgs.iter().filter_map(|&p| octree.find_id(p)){
          links.push(my_id); 
          links.push(id);
        }

        *index += 1;
      }
    }
  }*/
}

impl BoolLike for Node {
  fn union(self, other: Self) -> Self {
    match (self.0, other.0) {
      (Ok(cubes_a), Ok(cubes_b)) => {
        // calculate the union of the sub_cubes
        let cubes = (*cubes_a).union(*cubes_b);

        // if the new contain only empty or full blocks, return one of them
        match cubes.map_same(|a| a.map_same(|b| b.map_same(|node| node.get_state()))) {
          Some(State::Full) => Self(Err(Err(State::Full))),
          Some(State::Empty) => Self(Err(Err(State::Empty))),
          None => Self (Ok(Box::new(cubes)))
        }
      },
      // if one is empty, return the other
      (Err(Err(State::Empty)), b) => Self (b),
      (a, Err(Err(State::Empty))) => Self(a),
      // if one is full, return empty
      (Err(Err(State::Full)), b) => Self(Err(Err(State::Full))),
      (a, Err(Err(State::Full))) => Self(Err(Err(State::Full))),
      // if one is border, return it
      (Err(Ok(_)), _) => Self(Err(Ok(0))),
      (_, Err(Ok(_))) => Self(Err(Ok(0))),
    }
  }
  fn inter(self, other: Self) -> Self {
    match (self.0, other.0) {
      (Ok(cubes_a), Ok(cubes_b)) => {
        // calculate the intersection of the sub_cubes
        let cubes = (*cubes_a).union(*cubes_b);

        // if the new contain only empty or full blocks, return one of them
        match cubes.map_same(|a| a.map_same(|b| b.map_same(|node| node.get_state()))) {
          Some(State::Full) => Self(Err(Err(State::Full))),
          Some(State::Empty) => Self(Err(Err(State::Empty))),
          None => Self (Ok(Box::new(cubes)))
        }
      },
      // if one is full, return the other
      (Err(Err(State::Full)), b) => Self (b),
      (a, Err(Err(State::Full))) => Self(a),
      // if one is empty, return empty
      (Err(Err(State::Empty)), b) => Self(Err(Err(State::Empty))),
      (a, Err(Err(State::Empty))) => Self(Err(Err(State::Empty))),
      // if one is border, return it
      (Err(Ok(_)), _) => Self(Err(Ok(0))),
      (_, Err(Ok(_))) => Self(Err(Ok(0))),
    }
  }
  fn not(self) -> Self {
    match self.0 {
      Ok(cubes) => Self(Ok(Box::new((*cubes).not()))),
      Err(Err(State::Full)) => Self(Err(Err(State::Empty))),
      Err(Err(State::Empty)) => Self(Err(Err(State::Full))),
      Err(Ok(id)) => Self(Err(Ok(id)))
    }
  }
}

pub struct Octree {
  dim: V3,
  depth: usize,
  root: Node
}


impl Octree {
  pub fn new_test_sphere(depth: usize, center: V3, r: f32) -> Self {
    let dim =V3(128.0, 128.0, 128.0);
    let root = Node::approximate(V3(0.0, 0.0, 0.0), dim*0.5, &Sphere::new(center, r), depth);
    Self {dim, depth, root}
  }
  //pub fn triangulate(&mut self, point_array: Float32Array, triangles: Uint16Array
}
