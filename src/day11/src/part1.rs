use std::collections::VecDeque;
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

    let mut arr:Vec<Vec<char>> = content.lines().map(|s| s.chars().collect()).collect();
    arr = expand(arr);
    let n = arr.len();
    let m = arr.first().map_or(0, |row| row.len());

    let mut ans = 0;
    for i in 0..n {
        for j in 0..m {
            if arr[i][j] == '.' { continue; }
            let mut q:VecDeque<(usize, usize)> = VecDeque::new();
            let mut dis = vec![vec![-1; m]; n];
            q.push_back((i, j));
            dis[i][j] = 0;
            while !q.is_empty() {
                let (x, y) = *q.front().unwrap();

                if arr[x][y] == '#' {
                    ans += dis[x][y];
                }
                q.pop_front();
                let x = x as i32;
                let y = y as i32;
                let neighbours:Vec<(i32, i32)> = vec![(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)];
                for (nx, ny) in neighbours {
                    if nx < 0 || ny < 0 || nx >= n as i32 || ny >= m as i32 {continue;}
                    let nx = nx as usize;
                    let ny = ny as usize;
                    if dis[nx][ny] == -1 {
                        q.push_back((nx, ny));
                        dis[nx][ny] = dis[x as usize][y as usize] + 1;
                    }
                }
            }
        }
    }
    println!("{}", ans/2);
}

fn expand(arr: Vec<Vec<char>>) -> Vec<Vec<char>> {

    let n = arr.len();
    let m = arr.first().map_or(0, |row| row.len());
    let mut rows = vec![0; n];
    let mut cols = vec![0; m];
    for i in 0..n {
        for j in 0..m {
            let val = if arr[i][j] == '#' {1} else {0};
            rows[i] += val;
            cols[j] += val;
        }
    }
    let mut arr2:Vec<Vec<char>> = Vec::new();

    for i in 0..n {
        let mut row:Vec<char> = Vec::new();
        for j in 0..m {
            row.push(arr[i][j]);
            if cols[j] == 0 {
                row.push('.');
            }
        }
        arr2.push(row.clone());
        if rows[i] == 0 {
            arr2.push(row);
        }
    }
    arr2
}
