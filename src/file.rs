use std::{fs, io};

pub fn read_file(file_path: String) -> Result<String, io::Error> {
    fs::read_to_string(file_path)
}

pub fn save_to_file(content: String, path: String) {
    fs::write(path, content).unwrap();
}

fn get_arg_value(args: &Vec<String>, flags: Vec<String>) -> Result<String, ()> {
    // Find the correct flag in arguments
    let flag_index = args
        .iter()
        .position(|element| flags.contains(element))
        .unwrap();

    // Get the value for the file path flag
    let flag_value = args.get(flag_index + 1).unwrap();

    Ok(flag_value.to_string())
}

// args: ["--file", "/path/to/file.rs"]
pub fn get_file_path(args: &Vec<String>) -> Result<String, ()> {
    get_arg_value(args, vec!["--file".to_string(), "-f".to_string()])
}

// args: ["--out", "out.rs"]
pub fn get_output_path(args: &Vec<String>) -> Result<String, ()> {
    get_arg_value(args, vec!["--output".to_string(), "-o".to_string()])
}
