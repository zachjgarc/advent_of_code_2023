use crate::Regex;

pub fn run(input: &String) {
    let mut sum = 0u32;

    let mut card_count = Vec::<u32>::new();

    for (idx, line) in input.lines().enumerate() {
        if card_count.get(idx).is_none() { card_count.push(1); }

        let re = Regex::new(r"\:|\|").unwrap();
        let mut line_data = re.split(line);

        let win_nums = line_data.nth(1).unwrap().split_whitespace();
        let nums: Vec<&str> = line_data.next().unwrap().split_whitespace().collect();

        let win_count = win_nums.filter(|&win_num| nums.contains(&win_num)).count();

        for i in idx + 1..=idx + win_count {
            if i < card_count.len() {
                card_count[i] += card_count[idx];
            } else {
                card_count.push(1 + card_count[idx]);
            }
        }

        sum += card_count[idx];
    }

    println!("One: {sum}");
}