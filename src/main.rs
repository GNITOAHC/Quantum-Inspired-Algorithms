use serde_json;
use serde_json::Value;
use std::env;
use std::fs::File;
use std::io::Write;

pub struct Jxx {
    j: f64,  // J_{i,j} of x_i, x_j
    jl: f64, // J_{i,j} of x_i, x_j, but for layer between layer
    l: i32,  // Side length of the triangular lattice
    h: i32,  // Height of the triangular lattice
}

#[derive(Debug)]
pub enum SubLattice {
    RED,
    GREEN,
    BLUE,
}

mod node; // Contains the Node struct and it's implementation
use node::Node; // Use the Node struct

mod random; // Get the random number
use random::random_strength; // Use the random_strength function

mod hamiltonian; // Contains the hamiltonian_eff function
use hamiltonian::hamiltonian_eff; // Use the hamiltonian_eff function

/* 3D Triangular Lattice
 * (h: height, i: 2D i, j: 2D j)
 * current index: (h * L^2) + (i * L) + (j)
 */

static mut NODES: Vec<Node> = Vec::new();

// Main function
fn main() {
    let args: Vec<String> = env::args().collect(); // Get arguments
    let (mut use_random, mut debug_output): (bool, bool) = (false, false); // Add options to the program

    println!("{:?}", args);

    let mut jxx = Jxx {
        j: 1.0,
        jl: 1.0,
        l: 3,
        h: 3,
    };

    let mut i = 1;
    while i < args.len() {
        let val = || -> f64 {
            if i + 1 >= args.len() {
                panic!(
                    "Usage: {} [-J <J>] [-JL <JL>] [-L <L>] [-H <H>] [--use-random] [--debug-output]",
                    args[0]
                );
            }
            return args[i + 1].parse().expect("Failed to parse value");
        };

        if args[i] == "-J" {
            if val() <= 0.0 {
                println!("J should be greater than 0");
                return;
            }
            jxx.j = val();
        } else if args[i] == "-JL" {
            if val() <= 0.0 {
                println!("JL should be greater than 0");
                return;
            }
            jxx.jl = val();
        } else if args[i] == "-L" {
            if val() as i32 % 3 != 0 || val() as i32 <= 0 {
                println!("L should be multiple of 3 and greater than 0");
                return;
            }
            jxx.l = val() as i32;
        } else if args[i] == "-H" {
            if val() as i32 <= 0 {
                println!("H should be greater than 0");
                return;
            }
            jxx.h = val() as i32;
        } else if args[i] == "--use-random" {
            use_random = true;
        } else if args[i] == "--debug-output" {
            debug_output = true;
        }

        i += 2;
    }

    create_vector(&jxx);
    if use_random {
        random_strength(&jxx);
    }

    let fujitsu: Value = hamiltonian_eff(&jxx);
    write_json("./target/output.json", &fujitsu);

    if debug_output {
        print_node_info();
        debug_log(&fujitsu);
    }
}

fn create_vector(jxx: &Jxx) {
    #![allow(non_snake_case)]
    let L: i32 = jxx.l; // Side length of the triangular lattice
    let H: i32 = jxx.h; // Height of the triangular lattice
    let L2: i32 = jxx.l * jxx.l; // L^2

    let get_right = |h: i32, i: i32, j: i32| -> i32 {
        let _j: i32 = (j + 1) % L;
        return (h * L2) + (i * L) + (_j);
    };
    let get_bottom = |h: i32, i: i32, j: i32| -> i32 {
        let _i: i32 = (i + 1) % L;
        return (h * L2) + (_i * L) + (j);
    };
    let get_bottom_right = |h: i32, i: i32, j: i32| -> i32 {
        let _i: i32 = (i + 1) % L;
        let _j: i32 = (j + 1) % L;
        return (h * L2) + (_i * L) + (_j);
    };
    let get_layer_up = |h: i32, i: i32, j: i32| -> i32 {
        let _h: i32 = (h + 1) % H;
        return (_h * L2) + (i * L) + (j);
    };
    let get_sub_lattice = |index: i32| -> SubLattice {
        // Get the sub-lattice of the node.
        let sub_lattice: i32 = ((index / L) + index) % 3;
        match sub_lattice {
            0 => SubLattice::RED,
            1 => SubLattice::GREEN,
            2 => SubLattice::BLUE,
            _ => panic!("Error: sub_lattice is not 0, 1, or 2."),
        }
    };

    // Create the triangular lattice
    for h in 0..H {
        for i in 0..L {
            for j in 0..L {
                let index: i32 = (h * L2) + (i * L) + (j); // ex. let L = 6, current = 35 (h = 0, i = 5, j = 5);
                let right: i32 = get_right(h, i, j);
                let bottom: i32 = get_bottom(h, i, j);
                let btm_right: i32 = get_bottom_right(h, i, j);
                let layer_up: i32 = get_layer_up(h, i, j);
                let sub_lattice = get_sub_lattice(index);
                unsafe {
                    // Set the nodes
                    NODES.push(Node::new(
                        index,
                        right,
                        bottom,
                        btm_right,
                        layer_up,
                        sub_lattice,
                        &jxx,
                    ));
                }
            }
        }
    }
}

fn write_json(file_path: &str, fujitsu: &Value) -> () {
    let mut file = match File::create(file_path) {
        Ok(file) => file,
        Err(e) => {
            panic!("Error: {}", e);
        }
    };

    let formatted_data = match serde_json::to_string_pretty(&fujitsu) {
        Ok(data) => data,
        Err(e) => {
            panic!("Error: {}", e);
        }
    };

    if let Err(e) = file.write_all(formatted_data.as_bytes()) {
        panic!("Error: {}", e);
    }
}

fn print_node_info() {
    println!("========== NODES info ==========");
    unsafe {
        for i in 0..NODES.len() {
            NODES[i].print_info();
        }
    }
}

fn debug_log(fujitsu: &Value) {
    println!("========== DEBUG LOG ==========");
    let term_list = fujitsu["binary_polynomial"]["terms"].as_array().unwrap();
    for term in term_list {
        println!("cof: {}, poly: {}", term["coefficient"], term["polynomial"])
    }
    println!();
}
