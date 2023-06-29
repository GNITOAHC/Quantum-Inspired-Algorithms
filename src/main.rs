use serde_json;
use serde_json::{json, Value};
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
    let (mut use_random, mut debug_output, mut without_cycle): (bool, bool, bool) =
        (false, false, false); // Add options to the program
    let mut gamma: f64 = 1.0; // Gamma of the Hamiltonian (pass to metadata later)
    const TEN_DECIMAL_PLACES: f64 = 100000.0 * 100000.0; // Ten decimal places
    let mut time_limit_sec: Option<i64> = None; // Fujitsu request format (Default: 10, Min: 1, Max: 1800)

    println!("{:?}", args);

    let mut jxx = Jxx {
        j: 1.0,
        jl: 1.0,
        l: 3,
        h: 3,
    };

    let mut i = 1;
    while i < args.len() {
        let val = |i: &mut usize| -> f64 {
            *i += 1;
            if *i >= args.len() {
                panic!(
                    "Usage: {} [-J <J>] [-Gamma <Gamma>] [-L <L>] [-H <H>] [-T <T>] [--use-random] [--debug-output] [--without-cycle]",
                    args[0]
                );
            }
            return args[*i].parse().expect("Failed to parse value");
        };

        if args[i] == "-J" {
            let val = val(&mut i);
            if val <= 0.0 {
                println!("J should be greater than 0");
                return;
            }
            jxx.j = val;
        } else if args[i] == "-Gamma" {
            gamma = val(&mut i);
            if gamma == 0.0 {
                jxx.jl = 0.0;
            } else {
                let jl: f64 = -(0.5) * gamma.tanh().ln();
                // Ten decimal places
                jxx.jl = (jl * TEN_DECIMAL_PLACES).round() / TEN_DECIMAL_PLACES;
            }
        } else if args[i] == "-L" {
            let val: i32 = val(&mut i) as i32;
            if val % 3 != 0 || val <= 0 {
                println!("L should be multiple of 3 and greater than 0");
                return;
            }
            jxx.l = val;
        } else if args[i] == "-H" {
            let val = val(&mut i) as i32;
            if val <= 0 {
                println!("H should be greater than 0");
                return;
            }
            jxx.h = val;
        } else if args[i] == "-T" {
            let val = val(&mut i) as i64;
            if val < 1 || val > 1800 {
                println!("T should be between 1 and 1800");
                return;
            }
            time_limit_sec = Some(val);
        } else if args[i] == "--use-random" {
            use_random = true;
        } else if args[i] == "--debug-output" {
            debug_output = true;
        } else if args[i] == "--without-cycle" {
            without_cycle = true;
        }

        i += 1;
    }

    create_vector(&jxx);
    if use_random {
        random_strength(&jxx);
    }

    let mut fujitsu: Value = hamiltonian_eff(&jxx, without_cycle);
    write_request_format(&mut fujitsu, time_limit_sec);
    write_json("./target/input.json", &fujitsu);
    metadata("./target/metadata.json", &jxx, gamma, time_limit_sec);

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

fn write_request_format(fujitsu: &mut Value, time_limit_sec: Option<i64>) -> () {
    let da3 = fujitsu["fujitsuDA3"].as_object_mut().unwrap();
    if let Some(time_limit_sec) = time_limit_sec {
        da3.insert("time_limit_sec".to_string(), Value::from(time_limit_sec));
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

fn metadata(file_path: &str, jxx: &Jxx, gamma: f64, time_limit_sec: Option<i64>) -> () {
    let mut file = match File::create(file_path) {
        Ok(file) => file,
        Err(e) => {
            panic!("Error: {}", e);
        }
    };

    let mut meta = json!({});

    let data = meta.as_object_mut().unwrap();
    data.insert("Strength".to_string(), Value::from(jxx.j));
    data.insert("Layer_strength".to_string(), Value::from(jxx.jl));
    data.insert("Side_length".to_string(), Value::from(jxx.l));
    data.insert("Height".to_string(), Value::from(jxx.h));
    data.insert("Gamma".to_string(), Value::from(gamma));

    if let Some(time_limit_sec) = time_limit_sec {
        // If time_limit_sec is not None
        data.insert("Time_limit_sec".to_string(), Value::from(time_limit_sec));
    } else {
        data.insert("Time_limit_sec".to_string(), Value::from(10));
    }

    let formatted_data = match serde_json::to_string_pretty(&meta) {
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
