use std::collections::HashSet;
use std::fs::read_to_string;

pub(crate) fn run() {
    let file_path = "./input2.txt";
    let file_content = match read_to_string(file_path) {
        Ok(file_content) => file_content,
        Err(e) => {
            println!("Failed to read file {}", e);
            return;
        }
    };

    let v:Vec<&str> = file_content.lines().collect();
    let n = v.len();
    let mut cnt = vec![1;n];
    let mut ans = 0;
    for i in 0..n {
        let val = process(v[i]);
        for j in i+1..=i+val as usize {
            cnt[j] += cnt[i];
        }
        ans += cnt[i];
    }
    println!("{}", ans);
}

fn process(s: &str) -> u32 {
    let (_, s)  = s.split_at(8);
    let split_index = s.find("|").unwrap_or(0);
    let (first, second) = s.split_at(split_index);

    let hs:HashSet<i32> = first
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();
    let list:Vec<i32> = second
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    let ans = list.iter().filter(|s| hs.contains(s)).count() as u32;

    ans
}