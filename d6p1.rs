use std::io::Read;

fn main() -> std::io::Result<()> {
    let mut file = std::fs::File::open("input")?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;
    let mut ans = 0;
    let mut grid = vec![];
    let mut ops = vec![];

    let lines = s.lines().collect::<Vec<_>>();
    lines.iter().enumerate().for_each(|(i, line)| {
        let regex = regex::Regex::new(r" +").unwrap();
        let items = regex.split(line.trim());
        if i < lines.len() - 1 {
            grid.push(
                items
                    .into_iter()
                    .map(|i| i.parse::<usize>().unwrap_or(0))
                    .collect::<Vec<_>>(),
            );
        } else {
            ops = items.collect();
        }
    });

    for j in 0..grid[0].len() {
        let is_mul = ops[j] == "*";
        ans += (0..grid.len())
            .fold(if is_mul { 1 } else { 0 }, |acc, i| {
                if is_mul {
                    acc * grid[i][j]
                } else {
                    acc + grid[i][j]
                }
            });
    }

    println!("{ans}");

    Ok(())
}
