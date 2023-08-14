use serde_json;
use serde_json::{json, Value};
use std::fs::File;
use std::io::Write;

#[derive(Debug)]
pub struct Jxx {
    j: f64,     // J_{i,j} of x_i, x_j
    jl: f64,    // J_{i,j} of x_i, x_j, but for layer between layer
    l: i32,     // Side length of the triangular lattice
    h: i32,     // Height of the triangular lattice
    gamma: f64, // Gamma of the Hamiltonian
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

mod gamma_analysis; // Contains the analysis function
use gamma_analysis::analysis; // Use the analysis function

mod guidance_config; // Contains the use_guidance function
use guidance_config::use_guidance; // Use the use_guidance function

mod args; // Contains the Options struct

/* 3D Triangular Lattice
 * (h: height, i: 2D i, j: 2D j)
 * current index: (h * L^2) + (i * L) + (j)
 */

static mut NODES: Vec<Node> = Vec::new();

// Main function
fn main() {
    let options = args::get_options();

    // Check if arguments are for generting Gamma analysis data file
    // args ex: ["target/debug/fujitsu", "--gamma-analysis", "target/Gamma0.0/Strength1.0_Lattice12_12_1_Time10.json"]
    match &options.file_path {
        Some(file_path) => {
            analysis(file_path.clone());
            return;
        }
        None => {}
    }

    // Check if arguments are for generting guidance config file
    // args ex: ["target/debug/fujitsu", "--guidance-config", "target/Gamma0.0/Strength1.0_Lattice12_12_1_Time10.json"]
    match &options.guidance_path {
        Some(guidance_path) => {
            use_guidance(guidance_path.clone());
            return;
        }
        None => {}
    }

    let time_limit_sec = options.T;
    let (use_random, debug_output, without_cycle): (bool, bool, bool) = (
        options.use_random,
        options.debug_output,
        options.without_cycle,
    ); // Add options to the program

    let mut jxx = Jxx {
        j: 1.0,     // J_{i,j} of x_i, x_j
        jl: 1.0,    // J_{i,j} of x_i, x_j, but for layer between layer
        l: 3,       // Side length of the triangular lattice
        h: 3,       // Height of the triangular lattice
        gamma: 0.2, // Gamma of the Hamiltonian
    };

    args::get_jxx(&mut jxx, options);

    if jxx.gamma == 0.0 || jxx.h == 1 {
        // If Gamma equals 0, height should be 1 (If height is 1, Gamma should be 0)
        jxx.gamma = 0.0;
        jxx.jl = 0.0;
        jxx.h = 1;
    }
    println!("{:#?}", jxx);

    create_vector(&jxx);
    if use_random {
        random_strength(&jxx);
    }

    let mut fujitsu: Value = hamiltonian_eff(&jxx, without_cycle);
    write_request_format(&mut fujitsu, time_limit_sec);
    write_json("./target/input.json", &fujitsu);
    metadata("./target/metadata.json", &jxx, jxx.gamma, time_limit_sec);

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

fn write_request_format(fujitsu: &mut Value, time_limit_sec: Option<i32>) -> () {
    let da3 = fujitsu["fujitsuDA3"].as_object_mut().unwrap();
    if let Some(time_limit_sec) = time_limit_sec {
        da3.insert("time_limit_sec".to_string(), Value::from(time_limit_sec));
    }
    da3.insert("gs_level".to_string(), Value::from(100));
    da3.insert("gs_cutoff".to_string(), Value::from(100000));
    da3.insert("num_output_solution".to_string(), Value::from(1024));
}

pub fn write_json(file_path: &str, fujitsu: &Value) -> () {
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

fn metadata(file_path: &str, jxx: &Jxx, gamma: f64, time_limit_sec: Option<i32>) -> () {
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
        println!("cof: {}, poly: {}", term["c"], term["p"])
    }
    println!();
}
