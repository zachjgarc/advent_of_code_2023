use std::{io, io::Write};

fn main() {
    print!("Enter day # (1-25): ");
    io::stdout().flush().unwrap();
    let mut day = String::new();
    io::stdin().read_line(&mut day).expect("Failed to read line");

    match day.trim() {
        "1" => {
            let input = utils::fetch_input("../inputs/day_01.txt");
            day_01::one::run(&input);
            day_01::two::run(&input);
        },
        "2" => {
            let input = utils::fetch_input("../inputs/day_02.txt");
            day_02::one::run(&input);
            day_02::two::run(&input);
        },
        "3" => {
            let input = utils::fetch_input("../inputs/day_03.txt");
            day_03::one::run(&input);
            day_03::two::run(&input);
        },
        "4" => {
            let input = utils::fetch_input("../inputs/day_04.txt");
            day_04::one::run(&input);
            day_04::two::run(&input);
        },
        "5" => {
            let input = utils::fetch_input("../inputs/day_05.txt");
            day_05::one::run(&input);
            day_05::two::run(&input);
        },
        "6" => {
            let input = utils::fetch_input("../inputs/day_06.txt");
            day_06::one::run(&input);
            day_06::two::run(&input);
        },
        "7" => {
            let input = utils::fetch_input("../inputs/day_07.txt");
            day_07::one::run(&input);
            day_07::two::run(&input);
        },
        "8" => {
            let input = utils::fetch_input("../inputs/day_08.txt");
            day_08::one::run(&input);
            day_08::two::run(&input);
        },
        "9" => {
            let input = utils::fetch_input("../inputs/day_09.txt");
            day_09::one::run(&input);
            day_09::two::run(&input);
        },
        "10" => {
            let input = utils::fetch_input("../inputs/day_10.txt");
            day_10::one::run(&input);
            day_10::two::run(&input);
        },
        "11" => {
            let input = utils::fetch_input("../inputs/day_11.txt");
            day_11::one::run(&input);
            day_11::two::run(&input);
        },
        "12" => {
            let input = utils::fetch_input("../inputs/day_12.txt");
            day_12::one::run(&input);
            day_12::two::run(&input);
        },
        "13" => {
            let input = utils::fetch_input("../inputs/day_13.txt");
            day_13::one::run(&input);
            day_13::two::run(&input);
        },
        "14" => {
            let input = utils::fetch_input("../inputs/day_14.txt");
            day_14::one::run(&input);
            day_14::two::run(&input);
        },
        "15" => {
            let input = utils::fetch_input("../inputs/day_15.txt");
            day_15::one::run(&input);
            day_15::two::run(&input);
        },
        "16" => {
            let input = utils::fetch_input("../inputs/day_16.txt");
            day_16::one::run(&input);
            day_16::two::run(&input);
        },
        "17" => {
            let input = utils::fetch_input("../inputs/day_17.txt");
            day_17::one::run(&input);
            day_17::two::run(&input);
        },
        "18" => {
            let input = utils::fetch_input("../inputs/day_18.txt");
            day_18::one::run(&input);
            day_18::two::run(&input);
        },
        "19" => {
            let input = utils::fetch_input("../inputs/day_19.txt");
            day_19::one::run(&input);
            day_19::two::run(&input);
        },
        "20" => {
            let input = utils::fetch_input("../inputs/day_20.txt");
            day_20::one::run(&input);
            day_20::two::run(&input);
        },
        "21" => {
            let input = utils::fetch_input("../inputs/day_21.txt");
            day_21::one::run(&input);
            day_21::two::run(&input);
        },
        "22" => {
            let input = utils::fetch_input("../inputs/day_22.txt");
            day_22::one::run(&input);
            day_22::two::run(&input);
        },
        "23" => {
            let input = utils::fetch_input("../inputs/day_23.txt");
            day_23::one::run(&input);
            day_23::two::run(&input);
        },
        "24" => {
            let input = utils::fetch_input("../inputs/day_24.txt");
            day_24::one::run(&input);
            day_24::two::run(&input);
        },
        "25" => {
            let input = utils::fetch_input("../inputs/day_25.txt");
            day_25::one::run(&input);
            day_25::two::run(&input);
        },
        _ => eprintln!("Invalid day input."),
    }    
}
