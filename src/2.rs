use std::io::Read;

fn is_level_okay(cur: &[i64], skip: Option<usize>) -> bool {
    let mut is_okay = true;
    let (mut pos, mut neg) = (false, false);
    let mut prev = None;
    for i in 0..cur.len() {
        if matches!(skip, Some(ii) if ii == i) {
            continue;
        }
        if prev.is_none() {
            prev = Some(i);
            continue;
        }
        let diff = cur[i] - cur[prev.unwrap()];
        if diff > 0 {
            pos = true;
        } else {
            neg = true;
        }
        let e = diff.abs();
        if !(1..=3).contains(&e) {
            is_okay = false;
            break;
        }
        prev = Some(i);
    }
    is_okay &= pos ^ neg;
    is_okay
}

fn main() -> anyhow::Result<()> {
    let filepath = "tests/2/1.txt";
    let mut contents = String::new();
    let _ = std::fs::File::open(filepath)?.read_to_string(&mut contents)?;
    let levels = contents
        .split("\n")
        .map(|s| {
            s.split_ascii_whitespace()
                .map(|ns| ns.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let ans = levels
        .iter()
        .fold(0, |acc, cur| is_level_okay(cur, None) as usize + acc);

    println!("Part 1 ans: {ans}");

    let ans = levels.iter().fold(0, |acc, cur| {
        let mut is_okay = false;
        for skip in 0..cur.len() {
            if is_level_okay(cur, Some(skip)) {
                is_okay = true;
                break;
            }
        }
        acc + (is_okay as usize)
    });

    println!("Part 2 ans: {ans}");

    Ok(())
}
