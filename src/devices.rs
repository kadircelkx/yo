use colored::*;
use std::process::Command;

pub struct Device {
    pub index: i32,
    pub id: String,
    pub model: String,
}

pub fn get_devices() -> Vec<Device> {
    let mut devices = Vec::new();
    let output = Command::new("adb")
        .arg("devices")
        .output()
        .expect("Failed to execute ADB command.");

    let stdout = String::from_utf8(output.stdout).expect("Failed to convert output to string.");
    let mut count = 1;
    let mut devices_found = false;

    for line in stdout.lines() {
        if devices_found {
            if line.trim().is_empty() {
                break;
            }
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 && parts[1] == "device" {
                let index = count.clone();
                let id = parts[0];
                let output = Command::new("adb")
                    .arg("-s")
                    .arg(id)
                    .arg("shell")
                    .arg("getprop")
                    .arg("ro.product.model")
                    .output()
                    .expect("Unable to get product model.");
                let model = String::from_utf8(output.stdout)
                    .expect("Failed to convert product name to string.");
                devices.push(Device {
                    index,
                    id: String::from(id.trim()),
                    model: String::from(model.trim()),
                });
                count += 1;
            }
        } else if line.contains("List of devices attached") {
            devices_found = true;
        } else if !line.trim().is_empty() && !line.contains("daemon") {
            println!("Warning: unexpected output: {}", line);
        }
    }

    return devices;
}

pub fn print_device_info(index: i32, id: &str, model: &str) {
    println!(
        "{} {} ({})",
        index.to_string().bold().underline(),
        model,
        id
    );
}
