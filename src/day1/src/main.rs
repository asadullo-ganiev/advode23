use std::fs::read_to_string;

fn main() {
    let file_path = "./input3.txt";
    match read_to_string(file_path) {
        Ok(file_content) => {
            let sum:u32 = file_content.lines()
                .map(get_digits)
                .sum();
            println!("{}", sum);
        }
        Err(e) => println!("Failed to read file {}", e)
    }
}

fn find_digit(p0: &str, reverse : bool) -> u32 {
    let digits = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    if reverse {
        for (i, c) in p0.char_indices().rev() {
            if let Some(digit) = c.to_digit(10) {
                return digit;
            }

            for (j, &digit) in digits.iter().enumerate() {
                let ln = digit.len();
                if i + ln <= p0.len() && p0.get(i..i + ln).unwrap_or("") == digit {
                    return (j + 1) as u32;
                }
            }
        }
    } else {
        for (i, c) in p0.char_indices() {
            if let Some(digit) = c.to_digit(10) {
                return digit;
            }

            for (j, digit) in digits.iter().enumerate() {
                let ln = digit.len();
                if i + ln <= p0.len() && p0.get(i..i + ln).unwrap_or("") == *digit {
                    return (j + 1) as u32;
                }
            }
        }
    }
    0
}

fn get_digits(p0: &str) -> u32 {

    let first = find_digit(p0, false);
    let last = find_digit(p0, true);

    first * 10 + last
}
