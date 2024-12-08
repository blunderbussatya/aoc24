use std::io::Read;

fn is_valid(res: i64, opts: &[i64]) -> bool {
    let n = opts.len();
    let bs = n - 1;
    let mask = 1 << bs;

    for cm in 0..mask {
        let mut cur = opts[0];
        for j in 0..bs {
            let cur_elem = opts[j + 1];
            if (cm & (1 << j)) == 0 {
                cur += cur_elem;
            } else {
                cur *= cur_elem;
            }
            if cur > res {
                break;
            }
        }
        if cur == res {
            return true;
        }
    }
    false
}

fn concat_nums(a: i64, b: i64) -> i64 {
    let cs = format!("{a}{b}");
    cs.parse::<i64>().unwrap_or(i64::MAX)
}

fn is_valid_with_3_ops(res: i64, opts: &[i64]) -> bool {
    let n = opts.len();
    let bs = n - 1;
    let mask = 1 << bs;

    for cm in 0..mask {
        for sm in std::iter::successors(Some(cm), |m| {
            let nx = (m - 1) & cm;
            if *m == 0 {
                None
            } else {
                Some(nx)
            }
        }) {
            let mut cur = opts[0];
            for j in 0..bs {
                let cur_elem = opts[j + 1];
                match (sm & (1 << j), cm & (1 << j)) {
                    (0, 0) => cur += cur_elem,
                    (x, _) if x != 0 => cur = concat_nums(cur, cur_elem),
                    (0, y) if y != 0 => cur *= cur_elem,
                    _ => {
                        panic!("shouldn't reach here");
                    }
                }
                if cur > res {
                    break;
                }
            }
            if cur == res {
                return true;
            }
        }
    }
    false
}

fn main() -> anyhow::Result<()> {
    let filepath = "tests/7/1.txt";
    let mut contents = String::new();
    let _ = std::fs::File::open(filepath)?.read_to_string(&mut contents)?;

    let queries: Vec<(i64, Vec<i64>)> = contents
        .split('\n')
        .map(|s| {
            let (nm, opts) = s.split_once(':').unwrap();
            (
                nm.trim_ascii().parse().unwrap(),
                opts.trim_ascii()
                    .split(' ')
                    .map(|o| o.parse().unwrap())
                    .collect(),
            )
        })
        .collect();

    let ans = queries
        .iter()
        .map(|(res, opts)| match is_valid(*res, opts) {
            true => *res,
            false => 0,
        })
        .sum::<i64>();
    println!("Part 1 {ans:#?}");

    // Might take a while, release build is your friend
    let ans = queries
        .iter()
        .map(|(res, opts)| match is_valid_with_3_ops(*res, opts) {
            true => *res,
            false => 0,
        })
        .sum::<i64>();
    println!("Part 2 {ans:#?}");

    Ok(())
}
