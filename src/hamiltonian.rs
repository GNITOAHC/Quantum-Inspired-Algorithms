use crate::Jxx;
use crate::NODES;
use serde_json::{json, Map, Value};
use std::convert::TryFrom;

fn new_obj(cof: f64, vec: Vec<i32>) -> Map<String, Value> {
    let mut obj = Map::new();
    obj.insert("coefficient".to_string(), json!(cof));
    obj.insert("polynomial".to_string(), json!(vec));
    return obj;
}

fn get_front(i: i32, h: i32, jxx: &Jxx) -> Vec<(f64, Vec<i32>)> {
    #![allow(non_snake_case)]
    let L2: i32 = jxx.l * jxx.l; // L^2
    let idx: usize = usize::try_from((h * L2) + i).unwrap();
    let mut outer_vec: Vec<(f64, Vec<i32>)> = Vec::new();

    unsafe {
        let right_idx: usize = NODES[idx].right as usize;
        let right_j: f64 = NODES[idx].j_right;
        let bottom_idx: usize = NODES[idx].bottom as usize;
        let bottom_j: f64 = NODES[idx].j_bottom;
        let btm_right_idx: usize = NODES[idx].btm_right as usize;
        let btm_right_j: f64 = NODES[idx].j_btm_right;
        outer_vec.push((right_j, vec![idx as i32, right_idx as i32]));
        outer_vec.push((bottom_j, vec![idx as i32, bottom_idx as i32]));
        outer_vec.push((btm_right_j, vec![idx as i32, btm_right_idx as i32]));
    }

    outer_vec
}

fn get_back(idx: i32) -> Vec<(f64, Vec<i32>)> {
    let mut outer_vec: Vec<(f64, Vec<i32>)> = Vec::new();

    unsafe {
        let mut cur_idx = idx;
        loop {
            cur_idx = NODES[cur_idx as usize].layer_up;
            if cur_idx == idx {
                // Add negative sign to the coefficient
                outer_vec.push((0.0 - NODES[cur_idx as usize].j_layer_up, vec![idx, cur_idx]));
                break;
            }
            // Add negative sign to the coefficient
            outer_vec.push((0.0 - NODES[cur_idx as usize].j_layer_up, vec![idx, cur_idx]));
        }
    }

    outer_vec
}

#[allow(dead_code)]
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
                // Add negative sign to the coefficient
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
