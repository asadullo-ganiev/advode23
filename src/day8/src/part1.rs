use std::collections::HashMap;
use std::fs::read_to_string;

pub(crate) fn run() {
    let file_path = "./input3.txt";
    let content = match read_to_string(file_path) {
        Ok(file_content) => file_content,
        Err(e) => {
            println!("Failed to read file {}", e);
            return;
        }
    };

    let mut v:Vec<&str> = content.lines().collect();
    let directions = v[0];
    let mut v1:Vec<(&str,(&str, &str))>= v.iter().skip(2)
        .filter(|s| s.len() > 0)
        .map(|&s| {
            (substr(s, 0, 3), (substr(s, 7, 3), substr(s, 12, 3)))
        })
        .collect();

    let mut cur = "AAA";
    let mut hm:HashMap<&str, (&str, &str)> = HashMap::new();
    for (key, (left, right)) in v1 {
        hm.insert(key, (left, right));
    }
    let mut cur_step:usize = 0;
    let mut ans = 0;
    // println!("{}", hm.len());
    while cur != "ZZZ" {
        if directions.chars().nth(cur_step).unwrap() == 'L' {
            cur = hm[cur].0
        } else {
            cur = hm[cur].1
        }
        // println!("cur = {}", cur);
        cur_step = (cur_step + 1) % directions.len();
        ans += 1;
    }
    println!("{}", ans);
}

fn substr(s: &str, left: usize, sz: usize) -> &str {
    &s[left..left+sz]
}