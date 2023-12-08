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

    let mut cur:Vec<&str> = Vec::new();
    let mut hm:HashMap<&str, (&str, &str)> = HashMap::new();
    for (key, (left, right)) in v1 {
        if key.chars().last().unwrap() == 'A' {
            cur.push(key);
        }
        hm.insert(key, (left, right));
    }
    let mut cur_step:usize = 0;
    let mut cnt = 0;
    // println!("{}", hm.len());
    let mut ans:i64 = 0;
    let mut cycle_count = 0;
    while cnt < 1000000000 {
        if directions.chars().nth(cur_step).unwrap() == 'L' {
            cur = cur.into_iter().map(|s| hm[s].0).collect();
        } else {
            cur = cur.into_iter().map(|s| hm[s].1).collect();
        }
        cur_step = (cur_step + 1) % directions.len();
        cnt += 1;
        if cur.iter().any(|s| s.chars().last().unwrap() == 'Z') {
            ans = lcm(ans, cnt as i64);

            cycle_count += 1;
            if cycle_count == cur.len() { //This is not a general solution :(
                break;
            }

        }
    }
    println!("{}", ans);
}

fn lcm(p0: i64, p1: i64) -> i64 {
    if p0 == 0 {
        p1
    } else {
        p0 * (p1 / gcd(p0, p1))
    }
}

fn gcd(p0: i64, p1: i64) -> i64 {
    if p1 > 0 {gcd(p1, p0 % p1)} else {p0}
}

fn substr(s: &str, left: usize, sz: usize) -> &str {
    &s[left..left+sz]
}