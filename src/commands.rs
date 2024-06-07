use crate::devices::{get_devices, print_device_info};
use crate::utils::get_adb_version;
use colored::*;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process::{Command, Stdio};
use std::time::Instant;

pub fn version() {
    let adb_version = get_adb_version();
    println!("Yo: {}", env!("CARGO_PKG_VERSION"));
    println!("Android Debug Bridge: {}", adb_version.trim());
}

pub fn devices() {
    let devices = get_devices();
    if devices.is_empty() {
        println!("No devices found.");
        return;
    }

    println!("Devices");
    println!("-------------------------------------");
    for (index, device) in devices.iter().enumerate() {
        print_device_info((index + 1) as i32, &device.id, &device.model);
    }
}

pub fn connect(host: String) {
    println!("Trying to connect: {}", host.underline());
    let cmd = Command::new("adb")
        .arg("connect")
        .arg(&host)
        .output()
        .expect("Failed to execute ADB command.");
    if cmd.status.success() {
        println!("Successfully connected to: {}", host.underline());
    } else {
        println!("Failed To Connect: {}", host.underline());
        println!("ADB Message");
        println!("-------------------------------------");
        println!(
            "{}",
            String::from_utf8(cmd.stderr).expect("Failed to convert stderr to string.")
        );
    }
}

pub fn disconnect(host: String) {
    println!("Trying to disconnect: {}", host.underline());
    let cmd = Command::new("adb")
        .arg("disconnect")
        .arg(&host)
        .output()
        .expect("Failed to execute ADB command.");
    if cmd.status.success() {
        println!("Successfully disconnected from: {}", host.underline());
    } else {
        println!("Failed To Disconnect: {}", host.underline());
        println!("ADB Message");
        println!("-------------------------------------");
        println!(
            "{}",
            String::from_utf8(cmd.stderr).expect("Failed to convert stderr to string.")
        );
    }
}

pub fn push(from: String, to: String) {
    let metadata = fs::metadata(&from).expect("Failed to read file metadata.");
    let file_size = metadata.len();
    let file_extension = match from.split('.').last() {
        Some(ext) => ext,
        None => "Unknown",
    };
    let file_type = match metadata.file_type().is_dir() {
        true => "Directory",
        false => "File",
    };

    println!(
        "Trying to push from '{}' to '{}'",
        from.underline(),
        to.underline()
    );
    println!("-------------------------------------");
    println!(
        "Size: {} Bytes ({} Mb)",
        file_size.to_string().underline(),
        file_size / 1024 / 1024
    );
    println!("Extension: {}", file_extension.underline());
    println!("Type: {}", file_type.underline());
    println!("-------------------------------------");

    let cmd = Command::new("adb")
        .arg("push")
        .arg(&from)
        .arg(&to)
        .output()
        .expect("Failed to execute ADB command.");

    if cmd.status.success() {
        println!(
            "Successfully pushed '{}' ({}, {} bytes) to '{}'",
            from.underline(),
            file_type,
            file_size,
            to.underline()
        );
    } else {
        println!(
            "Failed to push '{}' to '{}'",
            from.underline(),
            to.underline()
        );
        println!("ADB Message");
        println!("-------------------------------------");
        println!(
            "{}",
            String::from_utf8(cmd.stderr).expect("Failed to convert stderr to string.")
        );
    }
}

pub fn pull(from: String, to: String) {
    println!(
        "Trying to pull from '{}' to '{}'",
        from.underline(),
        to.underline()
    );

    let cmd = Command::new("adb")
        .arg("pull")
        .arg(&from)
        .arg(&to)
        .output()
        .expect("Failed to execute ADB command.");

    if cmd.status.success() {
        println!(
            "Successfully pulled '{}' to '{}'",
            from.underline(),
            to.underline()
        );
    } else {
        println!(
            "Failed to pull '{}' to '{}'",
            from.underline(),
            to.underline()
        );
        println!("ADB Message");
        println!("-------------------------------------");
        println!(
            "{}",
            String::from_utf8(cmd.stderr).expect("Failed to convert stderr to string.")
        );
    }
}

pub fn shell() {
    println!("Starting interactive shell. Enter 'exit' to quit.");
    println!("CD doesn't work on here. Use something like this for running scripts in different locations: cd (path) && (command)");

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");

        if input.trim() == "exit" {
            break;
        }

        let start_time = Instant::now();
        let cmd = Command::new("adb")
            .arg("shell")
            .arg(&input)
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to execute ADB shell command.");

        let output = cmd.wait_with_output().expect("Failed to wait for command.");

        let duration = start_time.elapsed().as_micros();
        let code = output.status.code().unwrap_or(-1);

        println!("{}", String::from_utf8_lossy(&output.stdout));
        println!("Time: {}ms | Code: {}", duration, code);
    }
}

pub fn install(app_path: String) {
    println!("Installing application: {}", app_path);

    if !Path::new(&app_path).exists() {
        let app_name = app_path;
        println!(
            "Error: The specified APK file '{}' does not exist.",
            app_name
        );
        let google_play_url = format!(
            "https://play.google.com/store/apps/details?id={}",
            &app_name
        );
        let cmd = Command::new("adb")
            .arg("shell")
            .arg("am")
            .arg("start")
            .arg("-a")
            .arg("android.intent.action.VIEW")
            .arg("-d")
            .arg(&google_play_url)
            .output()
            .expect("Failed to execute ADB command");

        if cmd.status.success() {
            println!(
                "Successfully initiated download from Google Play Store for '{}'.",
                app_name
            );
        } else {
            println!(
                "Failed to initiate download from Google Play Store for '{}'.",
                app_name
            );
            println!("ADB Message:");
            println!("{}", String::from_utf8_lossy(&cmd.stderr));
        }
        return;
    }

    // Try to install the application
    let cmd = Command::new("adb")
        .arg("install")
        .arg(&app_path)
        .output()
        .expect("Failed to execute ADB install command.");

    if cmd.status.success() {
        println!("Application installed successfully.");
    } else {
        println!("Failed to install application.");
        println!("ADB Message:");
        println!("{}", String::from_utf8_lossy(&cmd.stderr));
    }
}
