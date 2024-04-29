pub struct Node {
    pub val: u32,
    pub x: usize,
    pub y: usize
}

pub struct NodeRange {
    pub val: u32,
    pub x: (usize, usize),
    pub y: (usize, usize)
}

pub fn fetch_input(path: &str) -> String {
    std::fs::read_to_string(path).expect("File not found")
}