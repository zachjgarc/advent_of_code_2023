pub fn run(input: &String) {
    let mut sum = 0_u32;

    for line in input.lines() {
        let mut line_data = line.chars().filter_map(|c| c.to_digit(10));
        let first = line_data.next().unwrap();
        let last = line_data.last().unwrap_or(first);

        sum += first * 10 + last;
    }

    println!("One: {}", sum);
}