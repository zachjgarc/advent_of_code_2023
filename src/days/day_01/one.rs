#[allow(unused_imports)]
use crate::utils::prelude::*;

pub fn run(input: &String) -> i32 {
	input.lines()
		.map(|line| {
            let mut line_data = line.chars()
                .filter_map(|c| c.to_digit(10));
            let first = line_data.next().unwrap();
            let last = line_data.last().unwrap_or(first);
            (first * 10 + last) as i32
        }).sum()
}