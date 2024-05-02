fn is_part_number(start_x: Option<usize>, x: usize, (prev, cur, next): (Option<&str>, &str, Option<&str>)) -> bool {
    let range = (start_x.unwrap() - if start_x == Some(0) { 0 } else { 1 })..(x + 1);
    let mut slice: &str;

    let mut is_part_number = false;
    if !is_part_number && prev != None {
        slice = &prev.unwrap()[range.clone()];
        is_part_number = slice.chars().any(|c| c != '.');
    }

    if !is_part_number {
        slice = &cur[range.clone()];
        is_part_number = slice.chars().any(|c| !c.is_digit(10) && c != '.');
    }

    if !is_part_number && next != None {
        slice = &next.unwrap()[range.clone()];
        is_part_number = slice.chars().any(|c| c != '.');
    }
    
    is_part_number
}

pub fn run(input: &String) {
    let mut sum: u32 = 0;

    let mut prev_line: Option<&str> = None;
    let mut next_line: Option<&str>;
    let mut buffer = String::new();

    let mut lines = input.lines().peekable();
    while let Some(line) = lines.next() {
        next_line = lines.peek().copied();

        let mut chars = line.chars().enumerate();
        let mut start_x = None;
        while let Some(value) = chars.next() {
            let (x,char): (usize, char) = value;

            if char.is_digit(10) {
                if start_x.is_none() {
                    start_x = Some(x);
                }
                buffer.push(char);
            } else if !buffer.is_empty() {
                if is_part_number(start_x, x, (prev_line, line, next_line)) {
                    sum += buffer.parse::<u32>().unwrap();
                }

                buffer.clear();
                start_x = None;
            }
        }

        if !buffer.is_empty() {
            if is_part_number(start_x, line.len() - 1, (prev_line, line, next_line)) {
                sum += buffer.parse::<u32>().unwrap();
            }

            buffer.clear();
        }

        prev_line = Some(line);
    }

    println!("One: {}", sum);
}