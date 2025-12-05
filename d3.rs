use std::io::Read;

fn main() -> std::io::Result<()> {
    let mut file = std::fs::File::open("input")?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;
    let mut ans = 0;

    const K: usize = 12; // 2 for r1

    s.lines().for_each(|line| {
        let mut stack = vec![];

        line.chars().enumerate().for_each(|(i, c)| {
            let d = c.to_digit(10).unwrap_or(0) as usize;
            while !stack.is_empty()
                && d > stack[stack.len() - 1]
                && stack.len() + line.len() - i > K
            {
                stack.pop();
            }
            if stack.len() < K {
                stack.push(d);
            }
        });

        let n = stack
            .into_iter()
            .reduce(|acc, curr| acc * 10 + curr)
            .unwrap();
        ans += n;
    });

    println!("{ans}");

    Ok(())
}

