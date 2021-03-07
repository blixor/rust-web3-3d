use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d as CTX;
use std::ops::{Index, IndexMut};
use itertools::iproduct;
mod shapes;

struct Point {
  x: f32,
  y: f32,
  id: u32,
}

// simple couple structure that you can index with a boolean
#[derive(Default)]
struct Couple<T>(T, T);

impl<T> Index<bool> for Couple<T> {
  type Output = T;
  
  fn index(&self, b: bool) -> &Self::Output {
    if b {&self.0} else {&self.1}
  }
}
impl<T> IndexMut<bool> for Couple<T> {
  fn index_mut(&mut self, b: bool) -> &mut Self::Output {
    if b {&mut self.0} else {&mut self.1}
  }
}

const FT: &[bool; 2] = &[false, true];


fn new_x(x: f32, w: f32, x_side: bool) -> f32 {if x_side {x + w/2.0} else {x}}

fn new_y(y: f32, h: f32, y_side: bool) -> f32 {if y_side {y + h/2.0} else {y}}


#[derive(Default)]
pub struct Node{
  subs: Option<Box<Couple<Couple<Node>>>>,
  p: Option<Point>
}

impl Node {
  fn empty() -> Node 
  { Node {subs: None, p: None}}
  
  fn size(&self) -> u32 {
    if self.p.is_some() {1}
    else if let Some(squares) = &self.subs {
      iproduct!(FT, FT).map(|(&bx, &by)| squares[bx][by].size()).sum()
    }
    else {0}
  }


  fn insert(&mut self, x_rect: f32, y_rect: f32, w: f32, h: f32, p: Point){

    match self.p {
      // if the node is empty, insert the value here 
      None => if self.subs.is_none() {self.p = Some(p); return},

      // if there is already a point: move it
      Some(Point {x, y, id}) => {
        self.p = None;

        let mut result : Box<Couple<Couple<Node>>> = Default::default();

        result[x > x_rect+w/2.0][y > y_rect+h/2.0].p = Some(Point {x, y, id});
        self.subs = Some(result);
      }};



      if let Some(squares) = &mut self.subs { 
   
        let x_side = p.x > x_rect+w/2.0;
        let y_side = p.y > y_rect+h/2.0;

        squares[x_side][y_side].insert(
          if x_side{x_rect+w/2.0} else {x_rect}, 
          if y_side {y_rect+h/2.0} else {y_rect}, 
          w/2.0, h/2.0, p
        )
    } 
  }

  fn find_in<S: shapes::Shape>(&self, x_rect: f32, y_rect: f32, w: f32, h: f32, shape: &S, points: &mut Vec<u32>){
    if shape.intersect_rect(x_rect, y_rect, w, h) {
      if let Some(Point{x, y, id}) = self.p {
        if shape.point_in(x, y) {
          points.push(id);
        }
      } 

      else if let Some(squares) = &self.subs {
        for (&bx, &by) in iproduct!(FT, FT) {
          squares[bx][by].find_in(
            new_x(x_rect, w, bx), 
            new_y(y_rect, h, by), 
            w/2.0, h/2.0, shape, points
          );
        }
      }
    }
  }

  fn draw(&self, x_rect: f32, y_rect: f32, w: f32, h: f32, ctx: &CTX){
    if let Some(squares) = &self.subs {
      for (&bx, &by) in iproduct!(FT, FT) {
        squares[bx][by].draw(
          new_x(x_rect, w, bx), 
          new_y(y_rect, h, by), 
          w/2.0, h/2.0, ctx
        );
      }
    }
    else {
      ctx.stroke_rect(x_rect as f64, y_rect as f64, w as f64, h as f64);
    }
  }
} 

#[wasm_bindgen]
pub struct QuadTree{
  width: f32,
  height: f32,
  root: Node,
}

#[wasm_bindgen]
impl QuadTree {
  #[wasm_bindgen(constructor)]
  pub fn new(width: f32, height: f32) -> Self{
    let root = Node::empty();
    QuadTree{width, height, root}
  }

  pub fn draw(&self, ctx: &CTX) {
    self.root.draw(0.0, 0.0, self.width, self.height, ctx);
  }

  pub fn size(&self) -> u32 {
    self.root.size()
  }

  pub fn clear(&mut self) {self.root = Node::empty()}

  pub fn insert(&mut self, x: f32, y: f32, id:u32){
    self.root.insert(0.0, 0.0, self.width, self.height, Point{x, y, id}); 
  }

  pub fn find_in_range(&self, x: f32, y: f32, w: f32, h: f32) -> Vec<u32> {
    let mut result = Vec::new();
    self.root.find_in(0.0, 0.0, self.width, self.height, &shapes::Rect::new(x, y, w, h), &mut result);
    result
  }
}
