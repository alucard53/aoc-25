use std::io::Read;

fn main() -> std::io::Result<()> {
    let mut file = std::fs::File::open("input")?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;
    let mut ans = 0;

    s.split(",").for_each(|range| {
        let v = range.trim().split("-").collect::<Vec<&str>>();
        let mut l = v[0].parse::<usize>().unwrap();
        let r = v[1].parse::<usize>().unwrap();


        while l <= r {
            let s = l.to_string();
            let n = s.as_bytes();
            let mut lps = vec![0; n.len()];
            let mut start = 0;
            let mut i = 1;

            while i < n.len() {
                if n[i] == n[start] {
                    start += 1;
                    lps[i] = start;
                    i += 1;
                } else if start != 0 {
                    start = lps[start - 1];
                } else {
                    lps[i] = 0;
                    i += 1;
                }
            }

            if start != 0 && n.len() % (n.len() - start) == 0 {
                ans += l;
            }
            l += 1;
        }
    });

    println!("{ans}");

    Ok(())
}
