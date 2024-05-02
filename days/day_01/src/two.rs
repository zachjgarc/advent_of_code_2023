pub fn run(input: &String) {
    let mut sum = 0_u32;

    for line in input.lines() {
        let line_data = parse_line(line);
        let first = line_data.first().unwrap();
        let last = line_data.last().unwrap();

        sum += first * 10 + last;
    }

    println!("One: {}", sum);
}

fn parse_line(line: &str) -> Vec<u32> {
    let nums: Vec<(&str, &str)> = vec!(
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
    );

    let mut new_line = String::from(line);

    for num in nums.iter() { new_line = new_line.replace(num.0, num.1) };

    new_line.chars().filter_map(|c| c.to_digit(10)).collect()
}