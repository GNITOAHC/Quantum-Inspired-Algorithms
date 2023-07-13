use crate::Jxx;
use crate::NODES;
use serde_json::{json, Map, Value};
use std::collections::HashMap;
use std::convert::TryFrom;

// Check direction of the nodes
enum Direction {
    Right,
    Bottom,
    BtmRight,
}

// Make new object for the fujitsu input json
fn new_obj(cof: f64, vec: &Vec<i32>) -> Map<String, Value> {
    let mut obj = Map::new();
    obj.insert("c".to_string(), json!(cof));
    if vec.len() == 1 && vec[0] == -1 {
        // Constant term
        return obj;
    }
    obj.insert("p".to_string(), json!(vec));
    return obj;
}

// Get the front part of the Hamiltonian function
fn get_front(i: i32, h: i32, jxx: &Jxx, without_cycle: bool) -> Vec<(f64, Vec<i32>)> {
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

        let iter_vec: Vec<(usize, f64, Direction)> = vec![
            (right_idx, right_j, Direction::Right),
            (bottom_idx, bottom_j, Direction::Bottom),
            (btm_right_idx, btm_right_j, Direction::BtmRight),
        ];

        // Drop the variables
        drop(right_idx);
        drop(right_j);
        drop(bottom_idx);
        drop(bottom_j);
        drop(btm_right_idx);
        drop(btm_right_j);

        for iter in iter_vec {
            let (iter_idx, iter_j, direction) = iter;

            // k: strength between two nodes (iter_j)
            // polynomial: 4k s_{i, n} s_{j, n} - 2k s_{i, n} - 2k s_{j, n} + k
            if without_cycle && is_cycle(idx, iter_idx, jxx.l, direction) {
                // check cycle
                continue;
            } else {
                outer_vec.push((4.0 * iter_j, vec![idx as i32, iter_idx as i32])); // 4k s_{i, n} s_{j, n}
                outer_vec.push((-2.0 * iter_j, vec![idx as i32])); // -2k s_{i, n}
                outer_vec.push((-2.0 * iter_j, vec![iter_idx as i32])); // -2k s_{j, n}
                outer_vec.push((iter_j, vec![-1])); // k (Constant term)
            }
        }
    }

    outer_vec
}

// Get the back part of the Hamiltonian function
fn get_back(idx: i32, without_cycle: bool) -> Vec<(f64, Vec<i32>)> {
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
            outer_vec.push((0.0 - cof4k, vec![cur_idx, next_idx]));
            outer_vec.push((0.0 - cof2k, vec![cur_idx]));
            outer_vec.push((0.0 - cof2k, vec![next_idx]));
            outer_vec.push((0.0 - cof_constant, vec![-1]));

            // Check if the next loop will reach the cycle
            if without_cycle && next_idx - 1 == idx {
                // When next_idx - 1 == idx, it means that next loop will reach the cycle
                break;
            }

            if next_idx == idx {
                // When next_idx == idx, it means that we have reached a cycle
                break;
            }

            cur_idx = next_idx;
            next_idx = NODES[cur_idx as usize].layer_up;
        }
    }

    outer_vec
}

pub fn hamiltonian_eff(jxx: &Jxx, without_cycle: bool) -> Value {
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

    // Build map for term consolidation
    let mut term_map: HashMap<Vec<i32>, f64> = HashMap::new();
    let mut constant_term: f64 = 0.0;

    for h in 0..height {
        for i in 0..L2 {
            let iter: Vec<(f64, Vec<i32>)> = get_front(i, h, jxx, without_cycle);
            for mut it in iter {
                it.1.sort();
                // it.0: f64 = coefficient, it.1: Vec<i32> = polynomial
                if it.1.len() == 1 && it.1[0] == -1 {
                    // constant term
                    constant_term += it.0;
                } else {
                    // term_map.insert(it.1, it.0);
                    if let Some((_k, v)) = term_map.get_key_value(&it.1) {
                        term_map.insert(it.1, v + it.0);
                    } else {
                        term_map.insert(it.1, it.0);
                    }
                }
            }
        }
    }

    if height == 1 {
        for (k, v) in term_map.iter() {
            term_list.push(Value::Object(new_obj(*v, k)));
        }
        if constant_term != 0.0 {
            term_list.push(Value::Object(new_obj(constant_term, &vec![-1])));
        }
        return fujitsu;
    }

    for i in 0..L2 {
        let iter: Vec<(f64, Vec<i32>)> = get_back(i, without_cycle);
        for mut it in iter {
            it.1.sort();
            // it.0: f64 = coefficient, it.1: Vec<i32> = polynomial
            if it.1.len() == 1 && it.1[0] == -1 {
                // constant term
                constant_term += it.0;
            } else {
                // term_map.insert(it.1, it.0);
                if let Some((_k, v)) = term_map.get_key_value(&it.1) {
                    term_map.insert(it.1, v + it.0);
                } else {
                    term_map.insert(it.1, it.0);
                }
            }
        }
    }

    for (k, v) in term_map.iter() {
        term_list.push(Value::Object(new_obj(*v, k)));
    }
    if constant_term != 0.0 {
        term_list.push(Value::Object(new_obj(constant_term, &vec![-1])));
    }

    fujitsu
}

// Check if the 2 nodes reach the cycle (without_cycle = true) (for the front part of the Hamiltonian function)
fn is_cycle(idx: usize, iter_idx: usize, side_length: i32, direction: Direction) -> bool {
    match direction {
        Direction::Right => {
            if iter_idx < idx {
                return true;
            }
        }
        Direction::Bottom => {
            if iter_idx != idx + side_length as usize {
                return true;
            }
        }
        Direction::BtmRight => {
            if iter_idx != idx + side_length as usize + 1 {
                return true;
            }
        }
    }
    return false;
}
