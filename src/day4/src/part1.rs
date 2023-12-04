use std::collections::HashSet;
use std::fs::read_to_string;

pub(crate) fn run() {
    let file_path = "./input2.txt";
    match read_to_string(file_path) {
        Ok(file_content) => {
            let sum:u32 = file_content.lines()
                .map(process)
                .sum();
            println!("{}", sum);
        }
        Err(e) => println!("Failed to read file {}", e)
    }
}

fn process(s: &str) -> u32 {
    let s1:String  = s.chars().skip(8).collect();
    let parts:Vec<&str> = s1.split("|").collect();
    let hs:HashSet<i32> = parts.first().unwrap().split(" ")
        .filter(|s| s.trim().len() > 0)
        .map(|s| s.trim().parse().unwrap_or(0)).collect();
    let list:Vec<i32> = parts.last().unwrap().split(" ")
        .filter(|s| s.trim().len() > 0)
        .map(|s| s.trim().parse().unwrap_or(0)).collect();

    let mut ans = 0;
    for item in list {
        if hs.contains(&item) {
            if ans == 0 {
                ans = 1;
            } else {
                ans *= 2;
            }
        }
    }
    ans
}