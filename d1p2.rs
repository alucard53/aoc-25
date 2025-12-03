use std::io::Read;

fn main() -> std::io::Result<()> {
    let mut file = std::fs::File::open("input")?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;

    let mut dial = 50;
    let mut ans = 0;
    s.lines().for_each(|line| {
        let mut rot = str::parse::<i32>(&line.chars().skip(1).collect::<String>()).unwrap();
        let dir = if &line[0..1] == "L" { -1 } else { 1 };

        let at_zero = dial == 0;
        ans += rot / 100;
        rot %= 100;
        dial = dial + (rot * dir);

        if dial <= 0 || dial >= 100 {
            ans += !at_zero as i32;
            dial = (dial + 100) % 100;
        }
    });
    println!("{ans}");

    Ok(())
}
