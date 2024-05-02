use std::{io, io::Write, time::{Duration, Instant}};

fn main() {
    print!("Enter day # (1-25): ");
    io::stdout().flush().unwrap();
    let mut day = String::new();
    io::stdin().read_line(&mut day).expect("Failed to read line");

    let mut ran = true;

    let mut start_time;
    let dur_one;
    let dur_two;
    match day.trim() {
        "1" => {
            let input = utils::fetch_input("./inputs/day_01.txt");

            start_time = Instant::now();
            day_01::one::run(&input);
            dur_one = start_time.elapsed();

            start_time = Instant::now();
            day_01::two::run(&input);
            dur_two = start_time.elapsed();
        },
        "2" => {
            let input = utils::fetch_input("./inputs/day_02.txt");

            start_time = Instant::now();
            day_02::one::run(&input);
            dur_one = start_time.elapsed();

            start_time = Instant::now();
            day_02::two::run(&input);
            dur_two = start_time.elapsed();
        },
        "3" => {
            let input = utils::fetch_input("./inputs/day_03.txt");

            start_time = Instant::now();
            day_03::one::run(&input);
            dur_one = start_time.elapsed();

            start_time = Instant::now();
            day_03::two::run(&input);
            dur_two = start_time.elapsed();
        },
        "4" => {
            let input = utils::fetch_input("./inputs/day_04.txt");

            start_time = Instant::now();
            day_04::one::run(&input);
            dur_one = start_time.elapsed();

            start_time = Instant::now();
            day_04::two::run(&input);
            dur_two = start_time.elapsed();
        },
        "5" => {
            let input = utils::fetch_input("./inputs/day_05.txt");

            start_time = Instant::now();
            day_05::one::run(&input);
            dur_one = start_time.elapsed();

            start_time = Instant::now();
            day_05::two::run(&input);
            dur_two = start_time.elapsed();
        },
        "6" => {
            let input = utils::fetch_input("./inputs/day_06.txt");

            start_time = Instant::now();
            day_06::one::run(&input);
            dur_one = start_time.elapsed();

            start_time = Instant::now();
            day_06::two::run(&input);
            dur_two = start_time.elapsed();
        },
        "7" => {
            let input = utils::fetch_input("./inputs/day_07.txt");

            start_time = Instant::now();
            day_07::one::run(&input);
            dur_one = start_time.elapsed();

            start_time = Instant::now();
            day_07::two::run(&input);
            dur_two = start_time.elapsed();
        },
        "8" => {
            let input = utils::fetch_input("./inputs/day_08.txt");

            start_time = Instant::now();
            day_08::one::run(&input);
            dur_one = start_time.elapsed();

            start_time = Instant::now();
            day_08::two::run(&input);
            dur_two = start_time.elapsed();
        },
        "9" => {
            let input = utils::fetch_input("./inputs/day_09.txt");

            start_time = Instant::now();
            day_09::one::run(&input);
            dur_one = start_time.elapsed();

            start_time = Instant::now();
            day_09::two::run(&input);
            dur_two = start_time.elapsed();
        },
        "10" => {
            let input = utils::fetch_input("./inputs/day_10.txt");

            start_time = Instant::now();
            day_10::one::run(&input);
            dur_one = start_time.elapsed();

            start_time = Instant::now();
            day_10::two::run(&input);
            dur_two = start_time.elapsed();
        },
        "11" => {
            let input = utils::fetch_input("./inputs/day_11.txt");

            start_time = Instant::now();
            day_11::one::run(&input);
            dur_one = start_time.elapsed();

            start_time = Instant::now();
            day_11::two::run(&input);
            dur_two = start_time.elapsed();
        },
        "12" => {
            let input = utils::fetch_input("./inputs/day_12.txt");

            start_time = Instant::now();
            day_12::one::run(&input);
            dur_one = start_time.elapsed();

            start_time = Instant::now();
            day_12::two::run(&input);
            dur_two = start_time.elapsed();
        },
        "13" => {
            let input = utils::fetch_input("./inputs/day_13.txt");

            start_time = Instant::now();
            day_13::one::run(&input);
            dur_one = start_time.elapsed();

            start_time = Instant::now();
            day_13::two::run(&input);
            dur_two = start_time.elapsed();
        },
        "14" => {
            let input = utils::fetch_input("./inputs/day_14.txt");

            start_time = Instant::now();
            day_14::one::run(&input);
            dur_one = start_time.elapsed();

            start_time = Instant::now();
            day_14::two::run(&input);
            dur_two = start_time.elapsed();
        },
        "15" => {
            let input = utils::fetch_input("./inputs/day_15.txt");

            start_time = Instant::now();
            day_15::one::run(&input);
            dur_one = start_time.elapsed();

            start_time = Instant::now();
            day_15::two::run(&input);
            dur_two = start_time.elapsed();
        },
        "16" => {
            let input = utils::fetch_input("./inputs/day_16.txt");

            start_time = Instant::now();
            day_16::one::run(&input);
            dur_one = start_time.elapsed();

            start_time = Instant::now();
            day_16::two::run(&input);
            dur_two = start_time.elapsed();
        },
        "17" => {
            let input = utils::fetch_input("./inputs/day_17.txt");

            start_time = Instant::now();
            day_17::one::run(&input);
            dur_one = start_time.elapsed();

            start_time = Instant::now();
            day_17::two::run(&input);
            dur_two = start_time.elapsed();
        },
        "18" => {
            let input = utils::fetch_input("./inputs/day_18.txt");

            start_time = Instant::now();
            day_18::one::run(&input);
            dur_one = start_time.elapsed();

            start_time = Instant::now();
            day_18::two::run(&input);
            dur_two = start_time.elapsed();
        },
        "19" => {
            let input = utils::fetch_input("./inputs/day_19.txt");

            start_time = Instant::now();
            day_19::one::run(&input);
            dur_one = start_time.elapsed();

            start_time = Instant::now();
            day_19::two::run(&input);
            dur_two = start_time.elapsed();
        },
        "20" => {
            let input = utils::fetch_input("./inputs/day_20.txt");

            start_time = Instant::now();
            day_20::one::run(&input);
            dur_one = start_time.elapsed();

            start_time = Instant::now();
            day_20::two::run(&input);
            dur_two = start_time.elapsed();
        },
        "21" => {
            let input = utils::fetch_input("./inputs/day_21.txt");

            start_time = Instant::now();
            day_21::one::run(&input);
            dur_one = start_time.elapsed();

            start_time = Instant::now();
            day_21::two::run(&input);
            dur_two = start_time.elapsed();
        },
        "22" => {
            let input = utils::fetch_input("./inputs/day_22.txt");

            start_time = Instant::now();
            day_22::one::run(&input);
            dur_one = start_time.elapsed();

            start_time = Instant::now();
            day_22::two::run(&input);
            dur_two = start_time.elapsed();
        },
        "23" => {
            let input = utils::fetch_input("./inputs/day_23.txt");

            start_time = Instant::now();
            day_23::one::run(&input);
            dur_one = start_time.elapsed();

            start_time = Instant::now();
            day_23::two::run(&input);
            dur_two = start_time.elapsed();
        },
        "24" => {
            let input = utils::fetch_input("./inputs/day_24.txt");

            start_time = Instant::now();
            day_24::one::run(&input);
            dur_one = start_time.elapsed();

            start_time = Instant::now();
            day_24::two::run(&input);
            dur_two = start_time.elapsed();
        },
        "25" => {
            let input = utils::fetch_input("./inputs/day_25.txt");

            start_time = Instant::now();
            day_25::one::run(&input);
            dur_one = start_time.elapsed();

            start_time = Instant::now();
            day_25::two::run(&input);
            dur_two = start_time.elapsed();
        },
        _ => {
            eprintln!("Invalid day input."); 
            ran = false;
            dur_one = Duration::new(0,0);
            dur_two = dur_one;
        },
    }

    if ran { println!("Part one in {:?}\nPart two in {:?}", dur_one, dur_two) };
}
