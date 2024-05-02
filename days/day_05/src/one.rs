pub fn run(input: &String) {
    let mut maps = input.split("\r\n\r\n");
    let seed_nums: Vec<u32> = maps.next().unwrap()
        .split_whitespace().filter_map(|val| val.parse().ok()).collect();
    println!("{:?}", seed_nums);
}