use std::cmp::max;
use std::collections::VecDeque;
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

    let mut arr:Vec<Vec<char>> = content.lines().map(|s| s.chars().collect()).collect();
    let n = arr.len();
    let m = arr.first().map_or(0, |row| row.len());
    let mut dis = vec![vec![-1; m]; n];

    let mut source = (0, 0);
    for i in 0..n {
        for j in 0..m {
            if arr[i][j] == 'S' {
                source = (i, j);
            }
        }
    }

    arr[source.0][source.1] = 'F';
    dis[source.0][source.1] = 0;
    let mut q:VecDeque<(usize, usize)> = VecDeque::new();
    q.push_back(source);
    let mut ans = 0;
    while !q.is_empty() {
        let (x, y) = *q.front().unwrap();
        ans = max(ans, dis[x][y]);
        q.pop_front();
        let mut neighbours:Vec<(usize, usize)> = Vec::new();
        match arr[x][y] {
            'F' => {
                neighbours.push((x + 1, y));
                neighbours.push((x, y + 1));
            }
            'J' => {
                neighbours.push((x - 1, y));
                neighbours.push((x, y - 1));
            }
            '7' => {
                neighbours.push((x + 1, y));
                neighbours.push((x, y - 1));
            }
            'L' => {
                neighbours.push((x - 1, y));
                neighbours.push((x, y + 1));
            }
            '-' => {
                neighbours.push((x, y - 1));
                neighbours.push((x, y + 1));
            }
            '|' => {
                neighbours.push((x - 1, y));
                neighbours.push((x + 1, y));
            }
            _ => {}
        }

        for (n_x, n_y) in neighbours {
            if n_x >= 0 && n_x < n && n_y >= 0 && n_y < m && dis[n_x][n_y] == -1 {
                q.push_back((n_x, n_y));
                dis[n_x][n_y] = dis[x][y] + 1;
            }
        }
    }
    println!("{}", ans);

}
