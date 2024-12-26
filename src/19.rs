use std::{collections::HashMap, io::Read};

fn validate(rules: &[&str], q: &str, ptr: usize, dp: &mut HashMap<usize, u64>) -> u64 {
    let cur = &q[ptr..];
    if cur.is_empty() {
        return 1;
    }
    if let Some(b) = dp.get(&ptr) {
        return *b;
    }
    let res = rules
        .iter()
        .map(|r| {
            if cur.starts_with(r) {
                validate(rules, q, ptr + r.len(), dp)
            } else {
                0
            }
        })
        .sum();
    dp.insert(ptr, res);
    res
}

fn main() -> anyhow::Result<()> {
    let filepath = "tests/19/1.txt";
    let mut contents = String::new();
    let _ = std::fs::File::open(filepath)?.read_to_string(&mut contents)?;
    let (rx, qx) = contents.split_once("\n\n").unwrap();
    let rules = rx.split(',').map(|s| s.trim_ascii()).collect::<Vec<_>>();

    let queries = qx.split('\n').collect::<Vec<_>>();
    let ans = queries
        .iter()
        .map(|q| {
            if validate(&rules, q, 0, &mut HashMap::new()) > 0 {
                1
            } else {
                0
            }
        })
        .sum::<u32>();
    println!("Part 1 ans: {ans:?}");

    let ans = queries
        .iter()
        .map(|q| validate(&rules, q, 0, &mut HashMap::new()))
        .sum::<u64>();
    println!("Part 2 ans: {ans:?}");
    Ok(())
}
