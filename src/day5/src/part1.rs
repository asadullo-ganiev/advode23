use std::fs::read_to_string;

pub(crate) fn run() {
    let file_path = "./input2.txt";
    let content = match read_to_string(file_path) {
        Ok(file_content) => file_content,
        Err(e) => {
            println!("Failed to read file {}", e);
            return;
        }
    };

    let v:Vec<&str> = content.lines().collect();
    let mut nums:Vec<i64> = Vec::new();
    let mut changed = vec![0; 100];
    for s in v {
        if s.starts_with("seeds") {
            nums = s.split(":").
                collect::<Vec<&str>>().get(1).unwrap().
                split(" ").filter_map(|s| s.parse().ok()).collect();
        } else if s.is_empty() {
            changed.fill(0);
        } else if s.chars().nth(0).unwrap().is_digit(10) {
            let f:Vec<i64> = s.split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();
            let left_source = f[1];
            let left_dest = f[0];
            let right_source = f[1] + f[2];
            for i in 0..nums.len() {
                if changed[i] == 0 && nums[i] >= left_source && nums[i] <= right_source{
                    nums[i] = left_dest + (nums[i] - left_source);
                    changed[i] = 1;
                }
            }
        }
    }
    println!("ans = {}", nums.iter().min().unwrap_or(&0))
}