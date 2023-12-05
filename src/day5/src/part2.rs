use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

pub(crate) fn run() {
    let file_path = "./input1.txt";
    let content = match read_to_string(file_path) {
        Ok(file_content) => file_content,
        Err(e) => {
            println!("Failed to read file {}", e);
            return;
        }
    };

    let v:Vec<&str> = content.lines().collect();
    let mut nums:HashSet<(i64, i64)> = HashSet::new();
    let mut nums3:HashSet<(i64, i64)> = HashSet::new();
    for s in v {
        if s.starts_with("seeds") {
            let x:Vec<i64> = s.split(":").
                collect::<Vec<&str>>().get(1).unwrap().
                split(" ").filter_map(|s| s.parse().ok()).collect();
            for i in 0..x.len() {
                if i % 2 == 1 {
                    nums.insert((x[i - 1], x[i - 1] + x[i] - 1));
                }
            }
        } else if s.is_empty() {
            for (_, val) in nums3.iter().enumerate() {
                nums.insert(*val);
            }
            nums3.clear();
        } else if s.chars().nth(0).unwrap().is_digit(10) {
            let f:Vec<i64> = s.split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();
            let left_source = f[1];
            let left_dest = f[0];
            let right_source = f[1] + f[2] - 1;
            let mut nums2:HashSet<(i64, i64)> = HashSet::new();
            for (_, val) in nums.iter().enumerate() {
                let (l1, r1) = *val;
                if not_intersects(l1, r1, left_source, right_source) {
                    nums2.insert((l1, r1));
                    continue;
                }
                if is_inside(l1, r1, left_source, right_source) {
                    let range1 = (l1, left_source - 1);
                    let range2 = (right_source + 1, r1);
                    let range3 = transform(left_source, right_source, left_source, left_dest);
                    check_and_add(&mut nums2, range1);
                    check_and_add(&mut nums2, range2);
                    check_and_add(&mut nums3, range3);
                } else if is_inside(left_source, right_source, l1, r1) {
                    let range3 = transform(l1, r1, left_source, left_dest);
                    check_and_add(&mut nums3, range3);
                } else if r1 <= right_source {
                    let range1 = (l1, left_source - 1);
                    let range2 = transform(left_source, r1, left_source, left_dest);
                    check_and_add(&mut nums2, range1);
                    check_and_add(&mut nums3, range2);
                } else if l1 >= left_source {
                    let range1 = (right_source + 1, r1);
                    let range2 = transform(l1, right_source, left_source, left_dest);
                    check_and_add(&mut nums2, range1);
                    check_and_add(&mut nums3, range2);
                }
            }
            nums.clear();
            for (_, val) in nums2.iter().enumerate() {
                nums.insert(*val);
            }
        }
    }

    println!("ans = {}", nums.iter().map(|(x, y)| x).min().unwrap_or(&0))
}

fn transform(l1: i64, r1: i64, lsource: i64, ldest:i64) -> (i64, i64) {
    (ldest + (l1 - lsource), ldest + (r1 - lsource))
}

fn check_and_add(p0: &mut HashSet<(i64, i64)>, p1: (i64, i64)) {
    if p1.0 <= p1.1 {
        p0.insert(p1);
    }
}

fn is_inside(l1: i64, r1: i64, l2: i64, r2: i64) -> bool {
    l2 >= l1 && r2 <= r1
}

fn not_intersects(l1:i64, r1:i64, l2:i64, r2:i64) -> bool {
    l1 > r2 || r1 < l2
}