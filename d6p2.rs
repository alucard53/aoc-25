use std::io::Read;

fn seek_next_op(index: &mut usize, s: &Vec<char>) {
    *index += 1;
    while *index < s.len() && !['*', '+'].contains(&s[*index]) {
        *index += 1;
    }
    // for last column move 1 further to simulate space between columns
    if *index == s.len() { 
        *index += 1;
    }
}

fn get_column_number(col: usize, grid: &Vec<Vec<char>>) -> usize {
    (0..grid.len() - 1).fold(0, |acc, i| {
        if let Some(digit) = grid[i][col].to_digit(10) {
            (acc * 10) + digit as usize
        } else {
            acc
        }
    })
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

    let mut j = 0;

    while j < n {
        let is_mul = grid[m - 1][j] == '*';
        let mut curr = if is_mul { 1 } else { 0 };
        let mut l = j;
        seek_next_op(&mut j, &grid[m - 1]);
        let r = j - 1;
        while l < r {
            let num = get_column_number(l, &grid);
            curr = if is_mul { curr * num } else { curr + num };
            l += 1
        }
        ans += curr;
    }

    println!("{ans}");

    Ok(())
}
