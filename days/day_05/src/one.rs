pub fn run(input: &String) {
    let mut raw_data = input.split("\r\n\r\n");
    let seed_nums = raw_data.next().unwrap()
        .split_whitespace()
        .filter_map(|val| val.parse::<i64>().ok());
    
    let mut maps = Vec::<Vec<(i64,i64,i64)>>::new();

    for raw_map_data in raw_data {
        maps.push(
            raw_map_data.lines().filter_map(|line| {
                let nums: Vec<i64> = line.split_whitespace()
                    .filter_map(|val| val.parse::<i64>().ok()).collect();
                if nums.len() == 3 {
                    Some( (nums[0], nums[1], nums[2]) )
                } else {
                    None
                }
            }).collect()
        );
    }

    let ans: i64 = seed_nums.map(|seed_num| get_location(seed_num, &maps)).min().unwrap();
    println!("One: {ans}");
}

fn get_location(mut seed_num: i64, maps: &Vec<Vec<(i64,i64,i64)>>) -> i64 {
    for map in maps {
        seed_num = single_map(seed_num,map);
    }
    
    seed_num
}

fn single_map(seed_num: i64, map: &Vec<(i64,i64,i64)>) -> i64 {
    let mut val = seed_num;

    for map_line in map.iter() {
        if seed_num >= map_line.1 && seed_num <= map_line.1 + map_line.2 - 1 { 
            val += (map_line.0 - map_line.1);
        }
    }
    
    val
}