extern crate libc;
extern crate serde;
extern crate serde_json;

use std::env;
use std::process::Command;
use serde_json::Value;
use std::fs::File;
use std::io::Read;

fn main() {
    let app_folder = "/Applications";

    let uid = unsafe { libc::geteuid() };
    assert_eq!(uid, 0, "The superuser(root) permission if required");
    
    if Command::new("fileicon").arg("--version").stdout(std::process::Stdio::null()).status().is_err() {
        eprintln!("fileicon is not installed, checkout https://github.com/mklement0/fileicon to install it");
        return;
    }

    let project_root: String;
    match env::current_exe() {
        Ok(path) => {
            project_root = path.parent().unwrap().parent().unwrap().parent().unwrap().to_str().unwrap().to_string();
        }
        Err(e) => {
            eprintln!("Get environment variable current_exe failed: {:?}", e);
            return;
        }
    }
    
    let json_path: String = format!("{project_root}/icon/app_list.json");
    let mut file: File = File::open(json_path).expect("Failed to open app_list.json file");
    let mut json_data = String::new();
    file.read_to_string(&mut json_data).expect("Failed to read app_list.json file");

    // Parse the JSON data into a serde_json::Value
    let json_value: Value = serde_json::from_str(&json_data).expect("Failed to parse JSON");

    // Assuming your JSON contains an array of strings, extract it into a vector
    let app_list: Vec<String>;
    if let Value::Array(string_array) = json_value {
        app_list = string_array
            .iter()
            .filter_map(|item| item.as_str().map(|s| s.to_owned()))
            .collect();
    } else {
        println!("JSON does not contain an array of strings");
        return;
    }

    for app_name in &app_list {
        let app_path = format!("{app_folder}/{app_name}.app");
        let icon_path = format!("{project_root}/icon/{app_name}.icns");

        let test_status = Command::new("fileicon")
            .arg("test")
            .arg(&app_path)
            .status()
            .expect("fileicon test command failed");
        
        if test_status.success() {
            println!("The icon of app {} is already set", app_name)
        }
        else {
            let set_status: Result<std::process::ExitStatus, std::io::Error> = Command::new("fileicon")
            .arg("set")
            .arg(&app_path)
            .arg(&icon_path)
            .status();
            match set_status {
                Ok(_) => (),
                Err(_) => eprintln!("Error when setting icon for app {}", app_name),
            };
        }
    }
}
