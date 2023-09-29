extern crate serde;
extern crate serde_json;

use std::env;
use std::process::Command;
use serde_json::Value;
use std::fs::File;
use std::io::Read;

fn main() {
    let app_folder = "/Applications";

    let project_root = match env::current_dir() {
        Ok(path) => path.to_str().unwrap().to_string(),
        Err(_) => {
            eprintln!("Error getting script path");
            return;
        }
    };
    
    if Command::new("fileicon").arg("--version").stdout(std::process::Stdio::null()).status().is_err() {
        eprintln!("fileicon is not installed, checkout https://github.com/mklement0/fileicon to install it");
        return;
    }    
    
    let json_path = format!("{project_root}/icon/app_list.json");
    let mut file = File::open(json_path).expect("Failed to open file");
    let mut json_data = String::new();
    file.read_to_string(&mut json_data).expect("Failed to read file");

    // Parse the JSON data into a serde_json::Value
    let json_value: Value = serde_json::from_str(&json_data).expect("Failed to parse JSON");

    // Assuming your JSON contains an array of strings, extract it into a vector
    let mut app_list: Vec<String> = Vec::new();
    if let Value::Array(string_array) = json_value {
        app_list = string_array
            .iter()
            .filter_map(|item| item.as_str().map(|s| s.to_owned()))
            .collect();

        // Now you have your vector of strings
        // println!("{:?}", string_vector);
    } else {
        println!("JSON does not contain an array of strings");
    }

    for app_name in &app_list {
        let app_path = format!("{app_folder}/{app_name}.app");
        let icon_path = format!("{project_root}/icon/{app_name}.icns");

        let status = Command::new("fileicon")
            .arg("set")
            .arg(&app_path)
            .arg(&icon_path)
            .status();

        match status {
            Ok(_) => (),
            Err(_) => eprintln!("Error setting icon for {}", app_name),
        }
    }
}
