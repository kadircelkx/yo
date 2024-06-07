use std::process::Command;

pub fn get_adb_version() -> String {
    let adb_version_output = if cfg!(windows) {
        Command::new("cmd")
            .args(&["/C", "adb version | findstr /R /C:\"^Android Debug Bridge version\" | awk \"{print $4}\""])
            .output()
    } else {
        Command::new("bash")
            .args(&["-c", "adb version | sed -n '1p' | awk '{print $5}'"])
            .output()
    };

    let adb_version = String::from_utf8(
        adb_version_output
            .expect("Failed to get adb version.")
            .stdout,
    )
    .expect("Failed to convert adb version from utf8 vec to string.");
    return adb_version;
}
