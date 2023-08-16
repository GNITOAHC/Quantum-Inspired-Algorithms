use serde_json::Value;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub fn use_guidance(guidance_path: String) {
    let (gamma, strength, length, height) = get_data(guidance_path.clone()); // Get data from file path

    let gamma: f64 = gamma.parse().unwrap(); // Convert gamma to f64
    let strength: f64 = strength.parse().unwrap(); // Convert strength to f64
    let length: i32 = length.parse().unwrap(); // Convert length to i32
    let height: i32 = height.parse().unwrap(); // Convert height to i32
    let get_meta = read_payload_from_file("./target/metadata.json").unwrap(); // Get meta file data
    if (gamma, strength, length, height)
        != (
            get_meta["Gamma"].as_f64().unwrap(),
            get_meta["Strength"].as_f64().unwrap(),
            get_meta["Side_length"].as_i64().unwrap() as i32,
            get_meta["Height"].as_i64().unwrap() as i32,
        )
    {
        println!("Meta data is not matched");
        return;
    } else {
        println!("Meta data is matched");
        println!(
            "gamma: {}, strength: {}, length: {}, height: {}",
            gamma, strength, length, height
        );
    }

    println!("Loading...");
    let lattice: Value = read_payload_from_file(guidance_path.clone()).unwrap(); // Get json file data
    let mut input: Value = read_payload_from_file("./target/input.json").unwrap(); // Get input file data

    let configs = lattice["qubo_solution"]["solutions"].as_array().unwrap();
    let random_config_index: usize = rand::random::<usize>() % configs.len() as usize;
    println!("random_config_index: {}", random_config_index);

    let da3 = input["fujitsuDA3"].as_object_mut().unwrap();

    match da3.get_mut("guidance_config") {
        None => {
            println!("guidance_config is None\nInserting...");
            da3.insert(
                "guidance_config".to_string(),
                configs[random_config_index]["configuration"].clone(),
            );
            crate::write_json("./target/input.json", &input);
        }
        Some(d) => {
            println!("guidance_config is not None\nUpdating...");
            let update_guidance = d.as_object_mut().unwrap();
            update_guidance.remove_entry("guidance_config");
            update_guidance.insert(
                "guidance_config".to_string(),
                configs[random_config_index]["configuration"].clone(),
            );
            crate::write_json("./target/input.json", &input);
            return;
        }
    }
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
    (gamma, strength, length, height)
}
