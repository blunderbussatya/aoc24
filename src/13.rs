use anyhow::anyhow;
use std::io::Read;
fn get_prob(s: &str) -> anyhow::Result<Vec<(i64, i64)>> {
    let sp = s.trim_ascii().split('\n').collect::<Vec<_>>();
    if sp.len() != 3 {
        return Err(anyhow!("3 elements not there after split {sp:#?}"));
    }
    let res = sp
        .into_iter()
        .map(|c| {
            let (p1, p2) = c.split_once(':').unwrap().1.split_once(',').unwrap();
            (
                p1.trim_ascii().split_at(2).1.parse().unwrap(),
                p2.trim_ascii().split_at(2).1.parse().unwrap(),
            )
        })
        .collect();
    Ok(res)
}

fn solve_prob(prob: &[(i64, i64)], add: i64) -> Option<i64> {
    let (a1, a2) = prob[0];
    let (b1, b2) = prob[1];
    let (c1, c2) = prob[2];
    let (c1, c2) = (-(add + c1), -(add + c2));

    let b1c2 = b1 * c2;
    let b2c1 = b2 * c1;

    let a1b2 = a1 * b2;
    let a2b1 = a2 * b1;

    let c1a2 = c1 * a2;
    let c2a1 = c2 * a1;

    let x = {
        let nm = b1c2 - b2c1;
        let dm = a1b2 - a2b1;
        if nm % dm != 0 {
            return None;
        }
        nm / dm
    };

    let y = {
        let nm = c1a2 - c2a1;
        let dm = a1b2 - a2b1;
        if nm % dm != 0 {
            return None;
        }
        nm / dm
    };

    Some(x * 3 + y)
}

fn main() -> anyhow::Result<()> {
    let filepath = "tests/13/1.txt";
    let mut contents = String::new();
    let _ = std::fs::File::open(filepath)?.read_to_string(&mut contents)?;
    let probs = contents
        .split("\n\n")
        .map(|s| get_prob(s).unwrap())
        .collect::<Vec<_>>();
    let ans = probs
        .iter()
        .map(|p| solve_prob(p, 0).unwrap_or_default())
        .sum::<i64>();

    println!("Part 1 {ans}");
    let ans = probs
        .iter()
        .map(|p| solve_prob(p, 10000000000000).unwrap_or_default())
        .sum::<i64>();

    println!("Part 2 {ans}");
    Ok(())
}
