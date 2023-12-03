use std::fs::read_to_string;

fn run() {
    let file_path = "./input2.txt";
    match read_to_string(file_path) {
        Ok(file_content) => {
            let mut n = 0;
            let mut m = 0;
            let mut arr:Vec<Vec<char>> = Vec::new();
            let mut used:Vec<Vec<bool>> = Vec::new();
            for (_, s) in file_content.lines().enumerate() {
                arr.push(s.chars().collect());
                m = s.len();
                used.push(vec![false; m]);
                n += 1;
            }

            for i in 0..n {
                for j in 0..m {
                    if arr[i][j] != '.' && !arr[i][j].is_digit(10) {

                        for dx in 0..3 {
                            for dy in 0..3 {

                                if let Some(inner_used) = used.get(i + dx-1) {
                                    if let Some(_) = inner_used.get(j+dy-1) {
                                        if arr[i+dx-1][j+dy-1].is_digit(10) {
                                            used[i+dx-1][j+dy-1] = true;
                                            println!("{} {} {}", dx as i32 -1, dy as i32-1, arr[i+dx-1][j+dy-1]);
                                        }
                                    }

                                }
                            }
                        }
                        println!();
                    }
                }
            }
            let mut ans = 0;
            let mut sum = 0;
            for i in 0..n {
                for j in 1..m {
                    if used[i][j - 1] && arr[i][j].is_digit(10) {
                        used[i][j] = true;
                    }
                }
                for j in (0..m-1).rev() {
                    if used[i][j + 1] && arr[i][j].is_digit(10) {
                        used[i][j] = true;
                    }
                }

                for j in 0..m {
                    if used[i][j] == true && arr[i][j].is_digit(10){
                        sum = sum * 10 + arr[i][j].to_digit(10).unwrap_or(0);
                    }
                    else {
                        ans += sum;
                        if sum > 0 {
                            println!("sum = {}", sum);
                        }
                        sum = 0;
                    }
                }
                if sum > 0 {ans += sum;}
                sum = 0;
            }
            println!("ans = {}", ans);
        }
        Err(e) => println!("Failed to read file {}", e)
    }
}