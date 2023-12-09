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
    let ans:i64 = content.lines()
        .map(|s| {
            let mut v:Vec<i64> = s.split(" ").filter_map(|s| s.parse().ok()).collect();
            let mut sum1 = 0;
            let mut cnt = 0;
            while !v.iter().all(|&x| x == 0) {
                cnt += 1;
                if cnt % 2 == 1 {
                    sum1 += v.first().unwrap();
                } else {
                    sum1 -= v.first().unwrap();
                }
                let mut v1:Vec<i64> = Vec::new();
                for i in 1..v.len() {
                    v1.push(v[i] - v[i - 1]);
                }
                v = v1;
            }
            sum1
        }).sum();

    println!("{}", ans);
}
