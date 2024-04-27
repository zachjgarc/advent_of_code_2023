pub fn run(input: &String) {
    let mut sum: u32 = 0;

    for line in input.lines() {
        let line_data = crate::parse(line);

        sum += line_data[0] * line_data[1] * line_data[2];
    }

    println!("{}", sum);
}