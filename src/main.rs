use std::env;
use std::fs::File;
use std::io;
use std::io::Read;
use std::str::FromStr;
extern crate serde;
extern crate serde_json;
use serde_json::Value;

pub struct Jxx {
    j: i32,  // J_{i,j} of x_i, x_j
    jl: i32, // J_{i,j} of x_i, x_j, but for layer between layer
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

/* 3D Triangular Lattice
 * (h: height, i: 2D i, j: 2D j)
 * current index: (h * L^2) + (i * L) + (j)
 */

static mut NODES: Vec<Node> = Vec::new();

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

    unsafe {
        for i in 0..NODES.len() {
            NODES[i].print_info();
        }
    }
}

// Main function
fn main() {
    let args: Vec<String> = env::args().collect(); // Get arguments

    println!("{:?}", args);

    let mut jxx = Jxx {
        j: 1,
        jl: 1,
        l: 3,
        h: 3,
    };

    let mut i = 1;
    while i < args.len() {
        if i + 1 >= args.len() {
            println!("Please enter value after parameter");
            println!("Usage: {} -J <J> -JL <JL> -L <L> -H <H>", args[0]);
            return;
        }

        let value = args[i + 1].parse().expect("Failed to parse value"); // Set the value of the parameter

        if args[i] == "-J" {
            if value <= 0 {
                println!("J should be greater than 0");
                return;
            }
            jxx.j = value;
        } else if args[i] == "-JL" {
            if value <= 0 {
                println!("JL should be greater than 0");
                return;
            }
            jxx.jl = value;
        } else if args[i] == "-L" {
            if value % 3 != 0 || value <= 0 {
                println!("L should be multiple of 3 and greater than 0");
                return;
            }
            jxx.l = value;
        } else if args[i] == "-H" {
            if value <= 0 {
                println!("H should be greater than 0");
                return;
            }
            jxx.h = value;
        }

        i += 2;
    }

    create_vector(&jxx);
    print_node_info();

    // read_json("input.json");

    // main_loop();
}

fn print_node_info() {
    println!("========== NODES info ==========");
    unsafe {
        for i in 0..NODES.len() {
            NODES[i].print_info();
        }
    }
}

#[allow(unused)]
fn read_json(file_path: &str) {
    // Open the file in read-only mode.
    let mut file: File = match File::open(file_path) {
        Ok(file) => file,
        Err(error) => {
            panic!("Problem opening the file: {:?}", error);
        }
    };

    let mut contents = String::new();
    if let Err(error) = file.read_to_string(&mut contents) {
        panic!("Problem reading the file: {:?}", error);
    }

    // Desirialize the JSON string into Value.
    let json: Value = match serde_json::from_str(&contents) {
        Ok(json) => json,
        Err(error) => {
            panic!("Problem parsing the file: {:?}", error);
        }
    };

    // Get the name and config from json
    let name = json.get("name").and_then(Value::as_str);
    let config = json.get("config").and_then(Value::as_array);

    // Extract the data from json
    match name {
        Some(value) => println!("name: {}", value),
        None => println!("name is not found"),
    }
    match config {
        Some(values) => {
            for config_item in values {
                let index = config_item.get("index").and_then(Value::as_i64);
                let spin = config_item.get("spin").and_then(Value::as_bool);
                let j_right = config_item.get("j_right").and_then(Value::as_f64);
                let j_bottom = config_item.get("j_bottom").and_then(Value::as_f64);
                let j_btm_right = config_item.get("j_btm_right").and_then(Value::as_f64);
                let j_layer_up = config_item.get("j_layer_up").and_then(Value::as_f64);
                println!("index: {:?}", index);
                match (index, spin, j_right, j_bottom, j_btm_right, j_layer_up) {
                    (Some(idx), Some(spn), Some(jr), Some(jb), Some(jbr), Some(jlu)) => {
                        println!(
                            "index: {}, spin: {}, j_right: {}, j_bottom: {}, j_btm_right: {}, j_layer_up: {}",
                            idx, spn, jr, jb, jbr, jlu
                        );
                        unsafe {
                            let idx = idx as usize;
                            NODES[idx].spin = spn;
                            NODES[idx].j_right = jr as f32;
                            NODES[idx].j_bottom = jb as f32;
                            NODES[idx].j_btm_right = jbr as f32;
                            NODES[idx].j_layer_up = jlu as f32;
                        }
                        print_node_info();
                    }
                    _ => println!("Invalid config item found"),
                }
            }
        }
        None => println!("config is not found"),
    }
    return;
}

#[allow(unused)]
fn parse_input<T: FromStr>(input: &str) -> T {
    input
        .parse()
        .unwrap_or_else(|_| panic!("Failed to parse input"))
}

#[allow(unused)]
fn main_loop() {
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // Parse the input
        let inputs: Vec<_> = input.split_whitespace().collect();

        // Extract the parsed inputs
        // let i: i32 = parse_input(&inputs[0]);
        // let f1: i32 = parse_input(&inputs[1]);
        // let f2: f64 = parse_input(&inputs[2]);
        let (front, back, strength): (i32, i32, f64) = (
            parse_input(&inputs[0]),
            parse_input(&inputs[1]),
            parse_input(&inputs[2]),
        );

        println!("{} {} {}", front, back, strength);

        break;
    }
}
