use utils::structs::XNodeRange;
use std::collections::HashMap;

pub fn run(input: &String) {
    let mut gears = HashMap::<(usize, usize), Vec<u32>>::new(); // keys are 1-indexed
    let mut numbers = Vec::<XNodeRange>::new();
    let mut buffer = String::new();

    for (y, line) in input.lines().enumerate() { 
        let mut chars = line.chars().enumerate().peekable();

        loop {
            match chars.peek() {
                Some(&(x, '*')) => {
                    gears.insert((x + 1, y + 1), Vec::new());
                    chars.next();
                }
                Some(&(start, '0'..='9')) => {
                    let mut end = start;
                    buffer.clear();
                    
                    while let Some((x, ch)) = chars.next_if(|&(_, c)| c.is_digit(10)) {
                        buffer.push(ch);
                        end = x;
                    }

                    numbers.push(XNodeRange {
                        val: buffer.parse().unwrap(),
                        x: (start + 1, end + 1), y: y + 1
                    });
                }
                Some(_) => { chars.next(); },
                None => break
            }
        }
    }

    for num in numbers {
        for x in num.x.0 - 1..=num.x.1 + 1 {
            for y in num.y - 1..=num.y + 1 {
                if let Some(data) = gears.get_mut(&(x,y)) {
                    data.push(num.val);
                }
            }
        }
    }

    let mut sum = 0;

    for data in gears.values() {
        if data.len() == 2 {
            sum += data[0] * data[1];
        }
    }

    println!("Two: {sum}");
}