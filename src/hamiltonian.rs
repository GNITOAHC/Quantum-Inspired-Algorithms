use crate::Jxx;
use crate::NODES;
use serde_json::{json, Map, Value};
use std::convert::TryFrom;

// Make new object for the fujitsu input json
fn new_obj(cof: f64, vec: Vec<i32>) -> Map<String, Value> {
    let mut obj = Map::new();
    obj.insert("coefficient".to_string(), json!(cof));
    if vec.len() == 1 && vec[0] == -1 {
        // Constant term
        return obj;
    }
    obj.insert("polynomial".to_string(), json!(vec));
    return obj;
}

// Get the front part of the Hamiltonian function
fn get_front(i: i32, h: i32, jxx: &Jxx) -> Vec<(f64, Vec<i32>)> {
    #![allow(non_snake_case)]
    let L2: i32 = jxx.l * jxx.l; // L^2
    let idx: usize = usize::try_from((h * L2) + i).unwrap();
    let mut outer_vec: Vec<(f64, Vec<i32>)> = Vec::new();

    unsafe {
        let (right_idx, right_j): (usize, f64) = (NODES[idx].right as usize, NODES[idx].j_right);

        let (bottom_idx, bottom_j): (usize, f64) =
            (NODES[idx].bottom as usize, NODES[idx].j_bottom);

        let (btm_right_idx, btm_right_j): (usize, f64) =
            (NODES[idx].btm_right as usize, NODES[idx].j_btm_right);

        // k: strengh between two nodes
        // polynomial: 4k s_{i, n} s_{j, n} - 2k s_{i, n} - 2k s_{j, n} + k
        outer_vec.push((4.0 * right_j, vec![idx as i32, right_idx as i32]));
        outer_vec.push((4.0 * bottom_j, vec![idx as i32, bottom_idx as i32]));
        outer_vec.push((4.0 * btm_right_j, vec![idx as i32, btm_right_idx as i32]));
        outer_vec.push((-2.0 * right_j, vec![right_idx as i32]));
        outer_vec.push((-2.0 * bottom_j, vec![bottom_idx as i32]));
        outer_vec.push((-2.0 * btm_right_j, vec![btm_right_idx as i32]));

        let outer_strength: f64 = right_j + bottom_j + btm_right_j;
        // outer_vec.push((-2.0 * outer_strength, vec![idx as i32])); // Combine all the outer terms
        outer_vec.push((-2.0 * right_j, vec![idx as i32]));
        outer_vec.push((-2.0 * bottom_j, vec![idx as i32]));
        outer_vec.push((-2.0 * btm_right_j, vec![idx as i32]));

        outer_vec.push((outer_strength, vec![-1])); // Constant term (With right, bottom, btm_right)
    }

    outer_vec
}

// Get the back part of the Hamiltonian function
fn get_back(idx: i32) -> Vec<(f64, Vec<i32>)> {
    let mut outer_vec: Vec<(f64, Vec<i32>)> = Vec::new();

    unsafe {
        let (mut cur_idx, mut next_idx) = (idx, NODES[idx as usize].layer_up);
        loop {
            let j_layer_up = NODES[cur_idx as usize].j_layer_up; // Get the strength of the bond

            // polynomial: 4k s_{i, n} s_{i, n+1} - 2k s_{i, n} - 2k s_{i, n+1} + k
            let cof4k: f64 = 4.0 * j_layer_up;
            let cof2k: f64 = -2.0 * j_layer_up;
            let cof_constant: f64 = j_layer_up;

            // Add negative sign to the coefficient (0.0 - cof)
            outer_vec.push((0.0 - cof4k, vec![cur_idx, cur_idx]));
            outer_vec.push((0.0 - cof2k, vec![cur_idx]));
            outer_vec.push((0.0 - cof2k, vec![next_idx]));
            outer_vec.push((0.0 - cof_constant, vec![-1]));

            if next_idx == idx {
                break;
            }

            cur_idx = next_idx;
            next_idx = NODES[cur_idx as usize].layer_up;
        }
    }

    outer_vec
}

pub fn hamiltonian_eff(jxx: &Jxx) -> Value {
    // H_{eff} = \sum{K s_{i, n} s_{j, n}} - \sum{K' s_{i, n} s_{i, n+1}}
    // sum1 -> i, j is a pair and n is the idx of layer; sum2 -> i is the idx of layer.
    #![allow(non_snake_case)]
    let L2: i32 = jxx.l * jxx.l; // L^2
    let height: i32 = jxx.h; // Height of the triangular lattice

    let mut fujitsu = json!({
        "fujitsuDA3": {},
        "binary_polynomial": {
            "terms": []
        }
    });

    let term_list = fujitsu["binary_polynomial"]["terms"]
        .as_array_mut()
        .unwrap();

    // println!("========== HAMILTONIAN ==========");

    for h in 0..height {
        for i in 0..L2 {
            let iter: Vec<(f64, Vec<i32>)> = get_front(i, h, jxx);
            for it in iter {
                // it.0: coefficient, it.1: polynomial
                term_list.push(Value::Object(new_obj(it.0, it.1)));
            }
        }
    }

    // println!("{:#}", fujitsu);

    if height == 1 {
        // println!("{:#}", fujitsu);
        return fujitsu;
    }

    for i in 0..L2 {
        let iter: Vec<(f64, Vec<i32>)> = get_back(i);
        for it in iter {
            // it.0: coefficient, it.1: polynomial
            term_list.push(Value::Object(new_obj(it.0, it.1)));
        }
    }

    // println!("{:#}", fujitsu);

    fujitsu
}
