use crate::Regex;

pub fn run(input: &String) {
    let mut sum = 0u32;

    for line in input.lines() {
        let re = Regex::new(r"\:|\|").unwrap();
        let mut line_data = re.split(line);

        let win_nums = line_data.nth(1).unwrap().split_whitespace();
        let nums: Vec<&str> = line_data.next().unwrap().split_whitespace().collect();

        let win_count = win_nums.clone().filter(|&win_num| nums.contains(&win_num)).count();

        if win_count > 0 { sum += 2u32.pow(win_count as u32 - 1); }
    }

    println!("One: {}", sum);
}