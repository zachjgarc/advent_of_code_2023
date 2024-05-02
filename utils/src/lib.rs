pub mod structs;
pub mod common;

pub fn fetch_input(path: &str) -> String {
    std::fs::read_to_string(path).expect("File not found")
}