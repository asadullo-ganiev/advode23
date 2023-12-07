use std::cmp::{max, Ordering};
use std::collections::HashMap;
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

    let mut v:Vec<(&str, i32)> = content.lines()
        .map(|s| {
            let parts:Vec<&str> = s.split(" ").collect();
            (parts[0], parts[1].parse().unwrap())
        }).collect();

    v.sort_by(|x, y| compare_hands(x.0, y.0));

    // println!("v =  {:?}", v);
    let ans:i64 = v.iter().enumerate().map(|(i, v)| {
        ((i + 1) as i64) * (v.1 as i64)
    }).sum();
    println!("{}", ans);
}

fn compare_hands(hand_a: &str, hand_b: &str) -> Ordering {
    let rank_a = rank(hand_a);
    let rank_b = rank(hand_b);
    let card_order = "AKQT98765432J";
    rank_a.cmp(&rank_b).then_with(|| {
        hand_a.chars().zip(hand_b.chars()).find_map(|(a, b)| {
            if a != b {
                let a_index = card_order.find(a).unwrap_or_default();
                let b_index = card_order.find(b).unwrap_or_default();
                Some(b_index.cmp(&a_index))
            } else {
                None
            }
        }).unwrap_or(Ordering::Equal)
    })
}

fn rank(s: &str) -> i32{
    let mut hm:HashMap<char, i32> = HashMap::new();
    let mut jokers = 0;
    for c in s.chars() {
        if c == 'J' {
            jokers += 1;
        } else {
            let count = hm.entry(c).or_insert(0);
            *count += 1
        }

    }
    if jokers == 5 {
        return 6;
    }

    let mut v:Vec<i32> = hm.into_values().collect();
    v.sort();
    let l = v.len();
    let first = v[0];
    let last = v[l - 1];
    if l == 1 && first + jokers == 5 {
        6
    } else if last + jokers == 4 {
        5
    } else if l <= 2 {
        4
    } else if last + jokers == 3 {
        3
    } else if last + jokers == 2 && max(0, jokers - (2 - last)) + v[l - 2] == 2 {
        2
    } else if last + jokers == 2 {
        1
    } else {
        0
    }
}
