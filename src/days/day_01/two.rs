#[allow(unused_imports)]
use crate::utils::prelude::*;

pub fn run(input: &String) -> i32 {
    let nums: Vec<(&str, &str)> = vec![
        ("zero", "z0o"),
        ("one", "o1e"),
        ("two", "t2o"),
        ("three", "t3e"),
        ("four", "f4r"),
        ("five", "f5e"),
        ("six", "s6x"),
        ("seven", "s7n"),
        ("eight", "e8t"),
        ("nine", "n9e")
    ];

    input.lines().map(|line| {
        let mut modified_line = line.to_string();
        nums.iter().for_each(|num| {
            modified_line = modified_line.replace(num.0, num.1);
        });

        let mut line_data = modified_line.chars()
            .filter_map(|c| c.to_digit(10));
        let first = line_data.next().unwrap();
        let last = line_data.last().unwrap_or(first);
        (first * 10 + last) as i32
    }).sum()
}