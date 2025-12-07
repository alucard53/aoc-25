use std::collections::VecDeque;
use std::io::Read;

enum Direction {
    LEFT,
    RIGHT,
    DOWN,
}

fn check_vis_and_add(
    i: usize,
    j: usize,
    dir: Direction,
    vis: &mut Vec<Vec<usize>>,
    q: &mut VecDeque<[usize; 2]>,
) {
    let (next_i, next_j) = match dir {
        Direction::LEFT => (i + 1, j - 1),
        Direction::RIGHT => (i + 1, j + 1),
        Direction::DOWN => (i + 1, j),
    };
    if vis[next_i][next_j] == 0 {
        q.push_back([next_i, next_j]);
    }
    vis[next_i][next_j] += vis[i][j];
}

fn main() -> std::io::Result<()> {
    let mut file = std::fs::File::open("input")?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;
    let mut ans = 0;

    let grid = s
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let m = grid.len();
    let n = grid[0].len();

    let mut q = VecDeque::new();
    let index = grid[0]
        .iter()
        .enumerate()
        .find(|(_, c)| **c == 'S')
        .unwrap();
    q.push_back([1, index.0]);
    let mut vis = vec![vec![0 as usize; n]; m];
    vis[1][index.0] = 1;

    while !q.is_empty() {
        let [i, j] = q.pop_front().unwrap();

        if i == m - 1 || j == n - 1 {
            continue;
        }

        if grid[i][j] == '^' {
            ans += 1;
            check_vis_and_add(i, j, Direction::LEFT, &mut vis, &mut q);
            check_vis_and_add(i, j, Direction::RIGHT, &mut vis, &mut q);
        } else {
            check_vis_and_add(i, j, Direction::DOWN, &mut vis, &mut q);
        }
    }

    println!("{ans} {}", vis[m - 1].iter().sum::<usize>());

    Ok(())
}
