use std::collections::VecDeque;
use std::io::Read;

const DIRS: [[i32; 2]; 8] = [
    [0, -1],
    [-1, -1],
    [-1, 0],
    [-1, 1],
    [0, 1],
    [1, 1],
    [1, 0],
    [1, -1],
];

fn get_surr_paper(grid: &Vec<Vec<u8>>, i: i32, j: i32) -> i32 {
    let m = grid.len() as i32;
    let n = grid.len() as i32;

    return DIRS
        .into_iter()
        .fold(0, |acc, [x, y]| {
            let i = i + x;
            let j = j + y;

            acc + if 0 <= i && i < m && 0 <= j && j < n {
                (grid[i as usize][j as usize] == '@' as u8) as i32
            } else {
                0
            }
        });
}

fn main() -> std::io::Result<()> {
    let mut file = std::fs::File::open("input")?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;
    let mut ans = 0;

    let mut grid = s
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .collect::<Vec<_>>();
    let m = grid.len() as i32;
    let n = grid[0].len() as i32;
    let mut q = VecDeque::new();

    for i in 0..m {
        for j in 0..n {
            if grid[i as usize][j as usize] == '.' as u8 {
                continue;
            }
            let surr = get_surr_paper(&grid, i, j);

            if surr < 4 {
                q.push_back(vec![i, j]);
            }
        }
    }

    while let Some(curr) = q.pop_front() {
        let i = curr[0];
        let j = curr[1];
        if grid[i as usize][j as usize] == '.' as u8 {
            continue;
        }

        let surr = get_surr_paper(&grid, i, j);

        if surr < 4 {
            ans += 1;
            grid[i as usize][j as usize] = '.' as u8;

            DIRS
                .into_iter()
                .for_each(|[x, y]| {
                    let i = i + x;
                    let j = j + y;
                    if 0 <= i && i < m && 0 <= j && j < n &&
                        grid[i as usize][j as usize] == '@' as u8
                    {
                        q.push_back(vec![i, j]);
                    }
                });
        }
    }

    println!("{ans}");

    Ok(())
}

