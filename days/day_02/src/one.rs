pub fn run(input: &str) {
    let mut sum: u32 = 0;

    for (line_num, line) in input.lines().enumerate() {
        let line_data = crate::parse(line);

        if line_data[0] <= 12 && line_data[1] <= 13 && line_data[2] <= 14 {
            sum += line_num as u32 + 1;
        }
    }

    println!("{}", sum);
}