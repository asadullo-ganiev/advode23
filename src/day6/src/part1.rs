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

    let v1:Vec<i64> = v[0].strip_prefix("Time: ").unwrap()
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();
    let v2:Vec<i64> = v[1].strip_prefix("Distance: ").unwrap()
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    let mut ans = 1;
    for i in 0..v1.len() {
        let time = v1[i];
        let mut cnt = 0;
        for hold in 0..time {
            if hold * (time - hold) > v2[i] {
                cnt += 1;
            }
        }
        ans *= cnt;
    }
    println!("{}", ans);
}