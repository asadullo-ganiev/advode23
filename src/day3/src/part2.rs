use std::fs::read_to_string;

pub(crate) fn run() {
    let file_path = "./input2.txt";
    let file_content = match read_to_string(file_path) {
        Ok(file_content) => file_content,
        Err(e) => println!("Failed to read file {}", e)
    };

    let mut arr:Vec<Vec<char>> = file_content.lines().map(|s| s.chars().collect()).collect();
    let n = arr.len();
    let m = arr.first().map_or(0, |row| row.len());
    let mut used = vec![vec![0; m]; n];
    let mut ans = 0;

    for i in 0..n {
        for j in 0..m {
            if arr[i][j] == '.' || arr[i][j].is_digit(10) {
                continue;
            }
            let mut cnt = 0;
            let mut pro = 1;
            for dx in 0..3 {
                for dy in 0..3 {
                    let ii:usize = i+dx-1;
                    let jj:usize = j+dy-1;

                    if let Some(inner_used) = used.get(ii) {
                        if let Some(_) = inner_used.get(jj) {
                            if arr[ii][jj].is_digit(10) && used[ii][jj] == 0{
                                used[ii][jj] = 1;
                                let mut left = jj;
                                let mut right = jj;
                                while left >= 1 && arr[ii][left - 1].is_digit(10) {
                                    left -= 1;
                                }
                                while right + 1 < m && arr[ii][right + 1].is_digit(10) {
                                    right += 1;
                                }
                                let mut sum = 0;
                                for id in left..right+1 {
                                    used[ii][id] = 1;
                                    sum = sum* 10 + arr[ii][id].to_digit(10).unwrap_or(0);
                                }
                                pro *= sum;
                                cnt += 1;
                            }
                        }
                    }
                }
            }
            if cnt == 2 {
                ans += pro;
            }

        }
    }
    println!("{}", ans);
}