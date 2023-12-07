use std::cmp::Ordering;
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

    v.sort_by(|x, y| {
        let rank_x = rank(x.0);
        let rank_y = rank(y.0);
        if rank_x > rank_y {
            Ordering::Greater
        } else if rank_y > rank_x {
            Ordering::Less
        } else {
            let sx = x.0;
            let sy = y.0;
            let s = "AKQJT98765432";
            for (i, c) in x.0.chars().enumerate() {
                let x1 = s.find(sx.chars().nth(i).unwrap()).unwrap();
                let y1 = s.find(sy.chars().nth(i).unwrap()).unwrap();
                if x1 < y1 {
                    return Ordering::Greater;
                } else if y1 < x1 {
                    return Ordering::Less;
                }
            }
            Ordering::Equal
        }
    });
    // println!("v =  {:?}", v);
    let ans:i32 = v.iter().enumerate().map(|(i, v)| {
        ((i + 1) as i32) * v.1
    }).sum();
    println!("{}", ans);
}

fn rank(s: &str) -> i32{
    let mut hm:HashMap<char, i32> = HashMap::new();
    for c in s.chars() {
        let count = hm.entry(c).or_insert(0);
        *count += 1
    }

    let mut v:Vec<i32> = hm.into_values().collect();
    v.sort();
    let l = v.len();
    if v[l - 1] == 5 {
        6
    } else if v[l - 1] == 4 {
        5
    } else if v[l - 1] == 3 && v[0] == 2 {
        4
    } else if v[l - 1] == 3 {
        3
    } else if v.len() == 3 && v[2] == 2 && v[1] == 2 {
        2
    } else if v[l - 1] == 2 {
        1
    } else {
        0
    }
}