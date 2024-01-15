use serde_json::Value;
use std::fs;
use std::str;

fn main() {
    let package_file: Vec<u8> = fs::read("package.json").unwrap();
    let mut file = read_json(&String::from_utf8_lossy(&package_file));

    let current_version: &str = file.get("version").unwrap().as_str().unwrap();

    println!("{:?}", current_version);
    let new_version: String = increment_version(current_version);

    file["version"] = new_version.into();

    println!("JSON {}", file);

    write_json(&file, "package.json").expect("Failed to write JSON to file");
}

fn read_json(raw_json: &str) -> Value {
    let parsed: Value = serde_json::from_str(raw_json).unwrap();
    return parsed;
}

fn write_json(json: &Value, filename: &str) -> std::io::Result<()> {
    let json_string = serde_json::to_string_pretty(json)?;
    fs::write(filename, json_string)
}

fn increment_version(version: &str) -> String {
    let parts: Vec<&str> = version.split('.').collect();

    let mut major: u32 = parts[0].parse::<u32>().unwrap_or(0);
    let mut minor: u32 = parts[1].parse::<u32>().unwrap_or(0);
    let mut patch: u32 = parts[2].parse::<u32>().unwrap_or(0);

    patch += 1;

    if patch == 10 {
        minor += 1;
        patch = 0;
    }

    if minor == 10 {
        major += 1;
        minor = 0;
    }

    let new_version: String = format!("{}.{}.{}", major, minor, patch);

    new_version
}
