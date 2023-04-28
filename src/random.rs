use rand::prelude::*;

pub fn get_random(max: f64) -> f64 {
    let mut rng = thread_rng();
    let result = rng.gen_range(0.0..max);
    return (result * 100.0).round() / 100.0;
}
