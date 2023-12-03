mod part2;
mod part1;

use std::collections::HashMap;
use std::fs::read_to_string;

fn main() {
    let file_path = "./input2.txt";
    match read_to_string(file_path) {
        Ok(file_content) => {
            let sum:i32 = file_content.lines()
                .map(process)
                .sum();
            println!("{}", sum);
        }
        Err(e) => println!("Failed to read file {}", e)
    }
}

fn process(s: &str) -> i32 {
    let parts = s.split(":").collect::<Vec<&str>>();
    let game:i32 = parts[0].split(" ").last().unwrap_or("0").parse().unwrap_or(0);

    let parts2 = parts[1].split(";").enumerate();
    let mut hm2:HashMap<&str, i32> = HashMap::new();
    for (_, part) in parts2 {
        let hm = process_parts(part);
        for (key, value) in hm {
            let old_val = *hm2.get(key).unwrap_or(&0);
            if old_val < value {
                hm2.insert(key, value);
            }
        }
    }
    hm2.values().product()
}

fn process_parts(s: &str) -> HashMap<&str, i32> {
    let mut hm:HashMap<&str, i32> = HashMap::new();
    let parts = s.split(",").enumerate();
    for (_, part) in parts {

        let parts2 = part.trim().split(" ").collect::<Vec<&str>>();
        hm.insert(parts2[1], parts2[0].parse().unwrap_or(0));
    }
    hm
}
