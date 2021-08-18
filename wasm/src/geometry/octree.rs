use std::ops::{Index, IndexMut};
use std::cell::RefCell;
use std::rc::Rc;
use array_init::array_init;

use super::V3;
use super::Dist;
use super::Range;
use super::random::rand_v3;

use super::console;

// bool structure: intersection, union and negation
trait BoolLike {
    fn union(a: Self, b: Self) -> Self;
    fn inter(a: Self, b: Self) -> Self;
    fn not(self) -> Self;
}


#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct CubeCorner (usize);

impl CubeCorner {
    fn bools(self) -> [bool; 3] {
        self.into()
    }
}

impl From<[bool; 3]> for CubeCorner {
    fn from(c: [bool; 3]) -> CubeCorner {
        CubeCorner(
            c.iter()
            .enumerate()
            .map(|(i, &b)| if b {1<<i} else {0})
            .sum()
        )
    }
}

impl From<CubeCorner> for [bool; 3] {
    fn from(t: CubeCorner) -> [bool; 3] {
        let id = t.0;
        array_init::array_init(
            |i| id>>i & 1 != 0
        )
    }
}

impl<T> Index<CubeCorner> for [T; 8] {
    type Output = T;
    fn index(&self, i: CubeCorner) -> &Self::Output {
        self.index(i.0)
    }
}



impl<T> IndexMut<CubeCorner> for [T; 8] {
    fn index_mut(&mut self, i: CubeCorner) -> &mut T {
        self.index_mut(i.0)
    }
}

pub fn sub_corner(corner_pos: V3, half_size: V3, corner: [bool; 3]) -> V3 {
    corner_pos + V3::new(
        if corner[0] {half_size.x} else {0.0},
        if corner[1] {half_size.y} else {0.0},
        if corner[2] {half_size.z} else {0.0},
    )
}

//              _     
// _ _  ___  __| |___ 
//| ' \/ _ \/ _` / -_)
//|_||_\___/\__,_\___|

type NodeIndex = [i32; 3];

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
enum NodeState {
    Inside,
    Outside
}

use NodeState::{Inside, Outside};

/// NodeState: can be inside or outside
impl NodeState {
    fn opposite(self) -> Self {
        match self {
            Inside => Outside,
            Outside => Inside,
        }
    }
}

impl Default for NodeState {
    fn default() -> Self {Outside}
}

#[derive(Debug)]
struct CellInfo<'octree> {
    cube_indices: RefCell<[Option<usize>; 8]>,
    neighbourgs: [[Result<&'octree CellInfo<'octree>, NodeState>; 2]; 3],
}

impl<'octree> Default for CellInfo<'octree> {
    fn default() -> CellInfo<'octree> {
        let neighbourgs = array_init(|_| array_init(|_| Err(Outside)));
        CellInfo {cube_indices:RefCell::new([None; 8]), neighbourgs}
    }
}

/// The Node object for the octree.
/// it can be either:
/// - a Cell (or leaf) that contain a refcell to a struct. This is the end of the recursion, at max
/// depth
/// - a State, `Inside` or `Outside`.
/// That means that this region of space is completely inside the shape or outside the shape
/// - 8 Subcubes (a cube is splited into 2 in the 3 directions of space)
enum Node<'octree> {
    Sub([Box<Node<'octree>>; 8]), 
    Cell(CellInfo<'octree>),
    Completely(NodeState),
}

impl <'octree> Node<'octree> where {
    /// Get state of the cell. 
    /// if completely outside or inside, return it.
    /// otherwise null
    fn get_state(&self) -> Option<NodeState> {
        if let Node::Completely(s) = self {
            Some(*s)
        }
        else {None}
    }

    /// index cell with 3 numbers
    fn index(&self, id: NodeIndex, depth: u8) -> Result<&'octree CellInfo, NodeState> {
        match self {
            Node::Sub(cubes) => {
                let m = 1 << (depth-1);
                let corner = map_array(id, |v| v >= m);
                cubes[CubeCorner::from(corner)]
                    .index(
                        map_array(id, |x| x%m),
                        depth-1
                    )
            }
            Node::Cell(x) => Ok(x),
            Node::Completely(s) => Err(*s),
        }
    }

    /// index cell with 3 numbers
    fn index_mut(&mut self, id: NodeIndex, depth: u8) -> Result<&'octree mut CellInfo, NodeState>{
        match self {
            Node::Sub(cubes) => {
                let m = 1 << (depth-1);
                let corner = map_array(id, |v| v >= m);
                cubes[CubeCorner::from(corner)]
                    .index_mut(
                        map_array(id, |x| x%m),
                        depth-1
                    )
            }
            Node::Cell(x) => Ok(x),
            Node::Completely(s) => Err(*s),
        }
    }
    /// get all references to the cells with corresponding index
    /// useless ? 
    fn get_cells_with_indices(&'octree self, id: NodeIndex, depth: u8, result: &mut Vec<(NodeIndex, &'octree CellInfo<'octree>)>) {
        match self {
            Node::Sub(cubes) => {
                let m = 1 << (depth-1);
                for i in 0..8 {
                    let corner = CubeCorner(i).bools();
                    let new_id = array_init(|d| id[d] + if corner[d] {m} else {0});
                    cubes[i].get_cells_with_indices(new_id, depth-1, result);
                }
            }
            Node::Cell(x) => result.push((id, x)),
            _ => (),
        }
    }

    /// approximate a distance function.
    /// TODO: use a Range instead of 2 V3
    fn approximate(corner: V3, half_size: V3, shape: &impl Dist, depth: u8) -> Self {
        // approximate distance function with octree
        let center = corner + half_size;
        // calculate the distance from the center to the nearest point of the shape
        let dist = shape.dist(center);

        // calculate the size of the diagonal (from the center to a corner)
        let diag = half_size.norm();

        if dist > diag { 
            // if distance is greater than the diagonal, it is outside
            Node::Completely(Outside)
        }
        else if dist < -diag {
            // same thing with opposite sign: we are inside
            Node::Completely(Inside)
        }
        else {
            if depth == 0 {
                // if max depth, add the center of the cell as a leaf
                Node::Cell(CellInfo::default())
            }
            else {
                // Otherwise, generate 8 subcubes
                Node::Sub(array_init(|i|
                        Box::new(Node::approximate(
                                sub_corner(corner, half_size, CubeCorner(i).into()),
                                half_size.scale(0.5),
                                shape, 
                                depth-1)
                        )
                ))
            }
        }
    }
}


/// zip 2 arrays with a function.
/// Soon, map and zip will be part of stable-rust !!!
fn zip_array_with<T, S, F, const N: usize>(a: [T; N], b: [T; N], mut f: F) -> [S; N] 
where F: FnMut(T, T) -> S
{
    use array_init::from_iter;
    use std::array::IntoIter;
    let zip = IntoIter::new(a).zip(IntoIter::new(b));
    from_iter(zip
        .map(|(a, b)| f(a, b))
    ).unwrap()
}

/// map an array with a function.
/// this will also be part of rust soon
fn map_array<T, S, F, const N: usize>(a: [T; N], f: F) -> [S; N] 
where F: FnMut(T) -> S
{
    use array_init::from_iter;
    use std::array::IntoIter;
    let iter = IntoIter::new(a);
    from_iter(iter
        .map(f)
    ).unwrap()
}


impl<'octree> BoolLike for Node<'octree>
{
    fn union(a: Self, b: Self) -> Self {
        // union of 2 octree nodes at the same location
        match (a, b) {
            (Node::Sub(cubes_a), Node::Sub(cubes_b)) => {
                // calculate the union of the sub_cubes
                let cubes = zip_array_with(
                    cubes_a,
                    cubes_b, 
                    |a, b| Box::new(BoolLike::union(*a, *b))
                );

                let mut fusion = cubes[0].get_state();
                for c in &cubes {
                    fusion = match (fusion, c.get_state()) {
                        (Some(Outside), Some(Outside)) => Some(Outside),
                        (Some(Inside), Some(Inside)) => Some(Inside),
                        _ => None,
                    };
                }

                // if the new contain only empty or full blocks, return one of them
                match fusion {
                    Some(state) => Node::Completely(state.opposite()),
                    None => Node::Sub(cubes)
                }
            },
            // if one is empty, return the other
            (Node::Completely(Outside), b) => b,
            (a, Node::Completely(Outside)) => a,
            // if one is full, return full
            (Node::Completely(Inside), _) => Node::Completely(Inside),
            (_, Node::Completely(Inside)) => Node::Completely(Inside),
            // if one is point, return it
            (Node::Cell(x), _) => Node::Cell(x),
            (_, Node::Cell(x)) => Node::Cell(x),
        }
    }

    fn inter(a: Self, b: Self) -> Self {
        // intersection of 2 octree nodes at the same location
        match (a, b) {
            (Node::Sub(cubes_a), Node::Sub(cubes_b)) => {
                // calculate the intersection of the sub_cubes
                let cubes = zip_array_with(
                    cubes_a,
                    cubes_b, 
                    |a, b| Box::new(BoolLike::inter(*a, *b))
                );

                let mut fusion = cubes[0].get_state();
                for c in &cubes {
                    fusion = match (fusion, c.get_state()) {
                        (Some(Outside), Some(Outside)) => Some(Outside),
                        (Some(Inside), Some(Inside)) => Some(Inside),
                        _ => None,
                    };
                }

                // if the new contain only empty or full blocks, return one of them
                match fusion {
                    Some(state) => Node::Completely(state.opposite()),
                    None => Node::Sub(cubes)
                }
            },
            // if one is full, return the other
            (Node::Completely(Inside), b) => b,
            (a, Node::Completely(Inside)) => a,
            // if one is empty, return empty
            (Node::Completely(Outside), _) => Node::Completely(Outside),
            (_, Node::Completely(Outside)) => Node::Completely(Outside),
            // if one is leaf, return it
            (Node::Cell(x), _) => Node::Cell(x),
            (_, Node::Cell(x)) => Node::Cell(x),
        }
    }

    fn not(self) -> Self {
        match self {
            Node::Sub(cubes) => Node::Sub(map_array(cubes, |c| Box::new(c.not()))),
            Node::Cell(x) => Node::Cell(x),
            Node::Completely(state) => Node::Completely(state.opposite()),
        }
    }
}


pub struct Octree<'octree> {
    range: Range,
    depth: u8,
    root: Node<'octree>,
    scale: V3,
}


impl <'octree> Octree<'octree> {
    fn index(&'octree self, id: NodeIndex) -> Result<&'octree CellInfo<'octree>, NodeState> {
        let max_id = 1 << self.depth;
        if id.iter().all(|&n| n >= 0 && n < max_id)
        {
            // if index is inside the octree
            self.root.index(id, self.depth)
        }
        else {
            // otherwise return error
            Err(Outside)
        }
    }

    fn index_mut(&'octree mut self, id: NodeIndex) -> Result<&'octree mut CellInfo<'octree>, NodeState> {
        let max_id = 1 << self.depth;
        if id.iter().all(|&n| n >= 0 && n < max_id)
        {
            // if index is inside the octree
            self.root.index_mut(id, self.depth)
        }
        else {
            // otherwise return error
            Err(Outside)
        }
    }

    /// give the position in space that correspond to an index in this octree
    fn index_to_point(&self, pos: NodeIndex) -> V3 {
        let vec_from_corner = V3::new(
            (pos[0] as f32) * self.scale.x,
            (pos[1] as f32) * self.scale.y,
            (pos[2] as f32) * self.scale.z,
        );
        vec_from_corner + self.range.smaller_corner
    }

    /// get all cells and indices inside the octree.
    fn get_cells_with_indices(&'octree self) -> Vec<(NodeIndex, &'octree CellInfo<'octree>)> {
        let mut result = Vec::new();
        self.root.get_cells_with_indices([0, 0, 0], self.depth, &mut result);
        result
    }

    /// approximate a distance function with an octree.
    /// `d`: struct that implement a distance function
    /// `range`: range of the octree (region of space in a tile)
    /// `depth`: depth you want (maximum 8)
    pub fn new_from_dist(d: impl Dist, range: Range, depth: u8) -> Self {
        let size = range.diagonal();
        let root = Node::approximate(range.smaller_corner, size.scale(0.5), &d, depth);
        let max_index = (1<<depth)-1;
        let scale = V3::new(
            size.x / (max_index as f32),
            size.y / (max_index as f32),
            size.z / (max_index as f32),
        );
        
        let mut result =  Self {range, depth, root, scale};
        
        // FIXME
        for (pos, _cell) in result.get_cells_with_indices(){ 
            let mut cell = result.index_mut(pos).unwrap();
            // get 6 neighbourgs of current cell
            cell.neighbourgs = array_init(
                |dim| array_init(
                    |side| result.index({
                        let mut id = pos.clone();
                        id[dim] = if side==1 {id[dim]+1} else {id[dim]-1};
                        id
                    })
                )
            );
        }

        result
    }

    /// triangulate octree with the surface-net algorithm
    pub fn triangulate(&self, point_array: &mut Vec<f32>, index_array: &mut Vec<u16>) {
        // get all cells in octree with the corresponding indices
        let all_cells = self.get_cells_with_indices();

        log!("number of cells: {:?}", all_cells.len());
        // for each cell in the octree, set index of all the corners
        for (cell_pos, x) in &all_cells {

            // modify temp value with new indices
            let indices = x.cube_indices.borrow_mut();
            let neighbourgs = x.neighbourgs;

            // (*temp_ref) = Some(array_init(
            //         |i| {
            //             let mut tmp_corner_index = None;

            //             for d in 0..3 {
            //                 // look at 3 neighbourgs cells (like up, down and front)
            //                 tmp_corner_index = tmp_corner_index.or(
            //                     // if there is a value defined, 
            //                     neighbourgs[d][if CubeCorner(i).bools()[d] {1} else {0}].as_ref()
            //                     .ok()
            //                     .and_then(|ref_corner| ref_corner.borrow().cube_indices)
            //                     .map(|corners| corners[i^(1<<d)])
            //                 );
            //             }
            //             match tmp_corner_index {
            //                 // if one of the neighbourgs cell have an index for this corner, return
            //                 // this number
            //                 Some(id) => id,
            //                 // otherwise, create a new id
            //                 None => {
            //                     let t = point_array.len()/12; 
            //                     let col = V3::new(0.3, 0.3, 0.3)+rand_v3().scale(0.1);
            //                     push_point!(point_array, self.index_to_point(*cell_pos), col, [0.0, 0.0, 0.0], [0.0, 0.0, 0.0]);
            //                     t
            //                 }
            //             }
            //         }));
        }

        // for (pos, x) in &all_cells {
        //     // create triangles for each cell
        //     for d in 0..3 {
        //         for &side in &[0, 1] {
        //             let pos_in_this_dir = {
        //                 let mut new_pos = pos.clone();
        //                 new_pos[d] += side*2-1;
        //                 new_pos
        //             };

        //             if let Err(Outside) = self.index(pos_in_this_dir){
        //                 // if neighbourg cell in this direction is outside, create triangles: 
        //                 let points_id_in_cube = x.borrow().cube_indices.unwrap();
        //                 let corners_squares = (0..8).filter(|&c| ((c as usize)>>d&1)==side as usize);
        //                 let points_id_in_square: Vec<_> = corners_squares.map(|i| points_id_in_cube[i]).collect();
        //                 push_index!( index_array, points_id_in_square.[0, 1, 2, 1, 3, 2]);
        //             }
        //         }
        //     }
        // }
    } 
}

#[cfg(test)]
mod tests {
    use super::{CubeCorner, Octree, Range, V3, map_array};
    use getrandom;

    #[test]
    fn conversions() {
        for i in 0..8 {
            let corner = CubeCorner(i);
            assert_eq!(corner, corner.bools().into())
        }
    }

    #[test]
    fn octree_indexing() {
        let range = Range::new(
            V3::new(-1.0, -1.0, -1.0),
            V3::new( 1.0,  1.0,  1.0),
        );
        let oct = Octree::new_from_dist(|v: V3| v.x*v.x+v.y+v.y-0.5, range, 4);

        for (_pos, ref_value) in oct.get_cells_with_indices() {
            let random_numbers = {
                let mut tmp = [0; 8];
                getrandom::getrandom(&mut tmp).unwrap();
                map_array(tmp, |x| x as usize)
            };
            ref_value.borrow_mut().cube_indices = Some(random_numbers);
        }

        for (pos, ref_value) in oct.get_cells_with_indices() {
            assert_eq!(
                oct.index(pos).unwrap().borrow().cube_indices, 
                ref_value.cube_indices
                );
        }
    }

    #[test]
    fn octree_triangulation() {
        let range = Range::new(
            V3::new(-1.0, -1.0, -1.0),
            V3::new( 1.0,  1.0,  1.0),
        );
        let oct = Octree::new_from_dist(|v: V3| v.x*v.x+v.y+v.y-0.5, range, 5);
        oct.triangulate(&mut Vec::new(), &mut Vec::new())
    }
}
