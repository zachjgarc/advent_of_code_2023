#[allow(unused_imports)]
use crate::utils::prelude::*;

pub fn run(input: &String) -> i32 {
	input.lines()
		.map(|line| line.chars().filter(|c| c.is_ascii_digit()).fold(|None|));
	-1
}