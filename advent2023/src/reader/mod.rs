use std::fs;

pub fn read_file(file_path: &str) -> String {
    println!("In file {}", file_path);

    fs::read_to_string(file_path)
        .expect("Something went wrong reading the file")
}