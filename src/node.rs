// Purpose: Contains the Node struct and its implementation
use crate::Jxx;
use crate::SubLattice;

pub struct Node {
    pub index: i32,              // Index of the node
    pub right: i32,              // 2D index of the right node
    pub bottom: i32,             // 2D index of the bottom node
    pub btm_right: i32,          // 2D index of the bottom right node
    pub layer_up: i32,           // 3D index of the layer up node
    pub spin: bool,              // Spin of the node, true: up, false: down
    pub sub_lattice: SubLattice, // Sub-lattice color of the node
    pub j_right: f64,            // J_{i,j} of current node and right node
    pub j_bottom: f64,           // J_{i,j} of current node and bottom node
    pub j_btm_right: f64,        // J_{i,j} of current node and bottom right node
    pub j_layer_up: f64,         // J_{i,j} of current node and layer up node
}

impl Node {
    pub fn new(
        index: i32,
        right: i32,
        bottom: i32,
        btm_right: i32,
        layer_up: i32,
        sub_lattice: SubLattice,
        jxx: &Jxx,
    ) -> Node {
        let j_value = jxx.j as f64; // Default J_{i,j} value
        Node {
            index,
            right,
            bottom,
            btm_right,
            layer_up,
            spin: false,
            sub_lattice,
            j_right: j_value,
            j_bottom: j_value,
            j_btm_right: j_value,
            j_layer_up: jxx.jl as f64,
        }
    }
    pub fn print_info(&self) {
        println!(
            "index: {:>2}; right: {:>2}, {:>5}; bottom: {:>2}, {:>5}; btm_right: {:>2}, {:>5}; layer_up: {:>2}, {:>5}; spin: {:>5}; sub_lattice: {:?};",
            self.index,
            self.right,
            self.j_right,
            self.bottom,
            self.j_bottom,
            self.btm_right,
            self.j_btm_right,
            self.layer_up,
            self.j_layer_up,
            self.spin,
            self.sub_lattice,
        );
    }
}
