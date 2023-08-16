use num::complex::Complex;
use serde_json::Value;
use std::error::Error;
use std::f64::consts::{E, PI};
use std::fmt;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

/*
* Config file structure
* {
*   "qubo_solution": {
*       "progress": [{"energy": -1.44, "time": 0.252}],
*       "result_status": true,
*       "solutions": [
*           {
*               "configuration": {
*                   "0": true,
*                   "1": false,
*                   ...
*               },
*               "energy": -144,
*               "frequency": 1
*           },
*           ...
*       ],
*       "timing": {"solve_time": "10840", "total_elapsed_time": "11024"},
*  },
*  "status": "Done"
* }
*/

#[derive(Debug)]
struct ComplexColorP {
    m_blue: Complex<f64>,
    m_black: Complex<f64>,
    m_red: Complex<f64>,
}

impl fmt::Display for ComplexColorP {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.m_blue, self.m_black, self.m_red)
    }
}

impl ComplexColorP {
    fn new(m_color_params: Vec<i32>, m_each_count: Vec<i32>) -> ComplexColorP {
        let m_blue = Complex::new(m_color_params[0] as f64 / m_each_count[0] as f64, 0.0);
        let m_black = Complex::new(m_color_params[1] as f64 / m_each_count[1] as f64, 0.0);
        let m_red = Complex::new(m_color_params[2] as f64 / m_each_count[2] as f64, 0.0);
        ComplexColorP {
            m_blue,
            m_black,
            m_red,
        }
    }
    fn get_order_parameter(&self) -> (Complex<f64>, f64) {
        // Define some usefull constants
        let imag_pi = Complex::new(0.0, (4.0 / 3.0) * PI);
        let math_e = Complex::new(E, 0.0);

        let order_parameter: Complex<f64> = (self.m_blue
            + self.m_black * (math_e.powc(imag_pi))
            + self.m_red * (math_e.powc(-1.0 * imag_pi)))
            / (3.0_f64).sqrt();

        // order_p = length of complex order_parameter
        let order_p: f64 = order_parameter.re.powf(2.0) + order_parameter.im.powf(2.0);
        (order_parameter, order_p)
    }
}

pub fn analysis(file_path: String) {
    let (gamma, strength, length, height) = get_data(file_path.clone()); // Get data from file path
    let num_length: i32 = length.parse().unwrap(); // Convert length to i32
    let num_height: i32 = height.parse().unwrap(); // Convert height to i32

    println!("Loading...");
    let lattice: Value = read_payload_from_file(file_path.clone()).unwrap(); // Get json file data
    println!("Calculating...");

    let configs = lattice["qubo_solution"]["solutions"].as_array().unwrap();

    println!("configs length: {}", configs.len());

    let mut analysis_data: Vec<String> = Vec::new();
    let mut skip_count: usize = 0;

    for (config_index, config) in configs.iter().enumerate() {
        let detail = config.as_object().unwrap();
        let energy = detail["energy"].as_f64().unwrap();

        // list_c6_orderp: Vec<(f64, f64, i32)> = (c6, order_p, layer)
        let list_c6_orderp: Vec<(f64, f64, i32)> =
            calc_c6_order_p(&detail["configuration"], num_length, num_height);

        // println!("list_c6_orderp length: {}", list_c6_orderp.len());

        for (c6, order_p, layer) in list_c6_orderp {
            if (c6, order_p) == (0.0, 0.0) {
                skip_count += 1;
                continue;
            } else {
                analysis_data.push(format!(
                    "{}\t{}\t{}\t{}\t{}",
                    c6, order_p, config_index, layer, energy
                ));
            }
        }

        // break; // tmp break in first config
    }

    println!("data length: {}", analysis_data.len());
    println!("skip count: {} ( c6 or order_p is 0.0, skip )", skip_count);

    assert_eq!(
        analysis_data.len() * num_height as usize + skip_count,
        configs.len() as usize
    );

    let target_dir = format!("./target/Gamma{}", gamma);
    std::fs::create_dir_all(&target_dir).unwrap(); // Create directory if not exists

    let target_file = format!(
        "{}/{}_{}_{}_{}.txt",
        target_dir, strength, length, length, height
    );

    println!("Saving to {}...", target_file);

    std::fs::File::create(target_file.clone()).unwrap();
    std::fs::write(target_file, analysis_data.join("\n")).unwrap();
}

fn calc_c6_order_p(config: &Value, length: i32, height: i32) -> Vec<(f64, f64, i32)> {
    let detail = config.as_object().unwrap();

    // BLUE, BLACK, RED
    let mut m_color_params: Vec<Vec<i32>> = vec![vec![0, 0, 0]; height as usize];
    let mut m_each_count: Vec<Vec<i32>> = vec![vec![0, 0, 0]; height as usize];

    for (key, value) in detail {
        let mut index = key.parse::<i32>().unwrap();
        let layer = index / (length * length);

        // layer should be less than height
        assert!(
            layer < height,
            "layer is over height: {} > {}",
            layer,
            height
        );

        index = index % (length * length);
        let remainder: i32 = ((index / length) + index) % 3;

        if layer != 0 {
            println!("layer: {}", layer);
        }

        m_each_count[layer as usize][remainder as usize] += 1;
        if value.as_bool().unwrap() {
            m_color_params[layer as usize][remainder as usize] += 1;
        } else {
            m_color_params[layer as usize][remainder as usize] -= 1;
        }

        // println!("m_color_params: {:?}", m_color_params);
    }

    let mut list_c6_orderp: Vec<(f64, f64, i32)> = Vec::new();

    for i in 0..height {
        let complex_color: ComplexColorP = ComplexColorP::new(
            m_color_params[i as usize].clone(),
            m_each_count[i as usize].clone(),
        );

        let (order_parameter, order_p) = complex_color.get_order_parameter();

        // Calculate c6
        let order_p_6: Complex<f64> = order_parameter.powf(6.0);
        if order_p_6 == Complex::new(0.0, 0.0) {
            list_c6_orderp.push((0.0, 0.0, 0_i32));
            continue;
        }
        let c6: f64 = order_p_6.re / order_p_6.norm();

        list_c6_orderp.push((c6, order_p, i as i32));
    }

    list_c6_orderp
}

fn read_payload_from_file<P: AsRef<Path>>(path: P) -> Result<Value, Box<dyn Error>> {
    // Open file in RO mode with buffer
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file
    let u = serde_json::from_reader(reader)?;

    Ok(u)
}

// Get data from file path (gamma, strength, length, height)
fn get_data(file_path: String) -> (String, String, String, String) {
    // file path ex: ./target/Gamma0.0/Strength1.0_Lattice12_12_1_Time10.json
    let v: Vec<&str> = file_path.split("Lattice").collect();
    // v: ["./target/Gamma0.0/Strength1.0_", "12_12_1_Time10.json"]
    let one: Vec<&str> = v[0].split("Gamma").collect();
    // one: ["./target/", "0.0/Strength1.0_"]
    let gamma: String = one[1].split("/").collect::<Vec<&str>>()[0].to_string();
    let strength: String = one[1].split("Strength").collect::<Vec<&str>>()[1]
        .split("_")
        .collect::<Vec<&str>>()[0]
        .to_string();
    let length: String = v[1].split("_").collect::<Vec<&str>>()[0].to_string();
    let height: String = v[1].split("_").collect::<Vec<&str>>()[2].to_string();
    println!(
        "gamma: {}, strength: {}, length: {}, height: {}",
        gamma, strength, length, height
    );
    (gamma, strength, length, height)
}
