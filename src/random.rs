use rand::prelude::*;
use crate::Jxx;
use crate::NODES;
use crate::print_node_info;

fn get_random(max: f64) -> f64 {
    let mut rng = thread_rng();
    let result = rng.gen_range(0.0..max);
    return (result * 100.0).round() / 100.0;
}

pub fn random_strength(jxx: &Jxx) {
    #![allow(non_snake_case)]
    let H: i32 = jxx.h; // Height of the triangular lattice.
    let L2: i32 = jxx.l * jxx.l; // L^2

    let mut rand_array = vec![0.0; (L2 * 3) as usize]; // Build a array of random numbers
    for i in 0..(L2 * 3) {
        rand_array[i as usize] = get_random(100.0);
    }

    let layer_rand = get_random(100.0);

    unsafe {
        // Set the nodes' strength to the random values (Only for the first layer)
        let (mut ndx, mut idx): (usize, usize) = (0, 0);
        loop {
            if ndx >= L2 as usize {
                break;
            }
            println!("idx = {}", idx);
            NODES[ndx].j_right = rand_array[idx];
            NODES[ndx].j_bottom = rand_array[idx + 1];
            NODES[ndx].j_btm_right = rand_array[idx + 2];
            NODES[ndx].j_layer_up = layer_rand;
            idx += 3;
            ndx += 1;
        }
    }

    unsafe {
        // Set the nodes' strength to the random values (For the other layers)
        for h in 1..H {
            // Set from height == 1
            let (mut ndx, mut idx): (usize, usize) = (0, 0);
            loop {
                if ndx >= L2 as usize {
                    break;
                }
                let nndx: usize = ((h * L2) as usize + ndx) as usize;
                NODES[nndx].j_right = rand_array[idx];
                NODES[nndx].j_bottom = rand_array[idx + 1];
                NODES[nndx].j_btm_right = rand_array[idx + 2];
                NODES[nndx].j_layer_up = layer_rand;
                idx += 3;
                ndx += 1;
            }
        }
    }

    print_node_info();
    return;
}
