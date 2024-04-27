pub mod one;
pub mod two;

pub fn parse(line: &str) -> Vec<u32> {
    let colors = vec!["red", "green", "blue"];
    let words: Vec<&str> = line.split_whitespace().collect();

    colors.iter().map(|&color| {
        words.windows(2)
            .filter(|&window| window[1].contains(color))
            .filter_map(|window| window[0].parse::<u32>().ok()) // all nums of a color
            .max().unwrap() // max num of that color
    }).collect()
}