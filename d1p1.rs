use std::io::Read;

fn main() -> std::io::Result<()> {
    let mut file = std::fs::File::open("input")?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;

    let mut dial = 50;
    let mut ans = 0;
    s.lines().for_each(|line| {
        let rot = str::parse::<i32>(&line.chars().skip(1).collect::<String>()).unwrap();
        let dir = if &line[0..1] == "L" { -1 } else { 1 };

        dial = ((dial + (rot * dir) % 100) + 100) % 100;
        ans += (dial == 0) as i32;
    });

    println!("{ans}");

    Ok(())
}
