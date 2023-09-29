use std::env;
use std::process::Command;

fn main() {
    let app_folder = "/Applications";
    let app_list = vec![
        "Visual Studio Code",
        "The Unarchiver",
        "Microsoft Word",
        "Microsoft PowerPoint",
        "Microsoft Excel",
        "Microsoft OneNote",
        "zoom.us",
        "Google Chrome",
        "AppCleaner",
        "24 Hour Wallpaper",
    ];

    if Command::new("fileicon").arg("--version").stdout(std::process::Stdio::null()).status().is_ok() {
        let script_path = match env::current_exe() {
            Ok(path) => path.parent().unwrap().to_str().unwrap().to_string(),
            Err(_) => {
                eprintln!("Error getting script path");
                return;
            }
        };

        for app_name in &app_list {
            let app_path = format!("{app_folder}/{app_name}.app");
            let icon_path = format!("{script_path}/../icon/{app_name}.icns");

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
    } else {
        eprintln!("fileicon is not installed, checkout https://github.com/mklement0/fileicon to install it");
    }
}
