use itertools::{Either, Itertools};
use std::{collections::HashMap, io::Read};

fn main() -> anyhow::Result<()> {
    let filepath = "tests/1/1.txt";
    let mut contents = String::new();
    let _ = std::fs::File::open(filepath)?.read_to_string(&mut contents)?;

    let (mut l1, mut l2): (Vec<_>, Vec<_>) = contents
        .split_ascii_whitespace()
        .map(|c| c.parse::<i64>().unwrap())
        .enumerate()
        .partition_map(|(i, v)| {
            if i % 2 == 0 {
                Either::Left(v)
            } else {
                Either::Right(v)
            }
        });

    l1.sort();
    l2.sort();

    let ans = std::iter::zip(l1.clone(), l2.clone()).fold(0, |acc, (x, y)| acc + (x - y).abs());

    println!("Part 1 ans: {ans}");

    let freq_map = l2.into_iter().fold(HashMap::new(), |mut map, val| {
        *map.entry(val).or_insert(0) += 1;
        map
    });

    let ans = l1
        .into_iter()
        .fold(0, |acc, e| (e * freq_map.get(&e).unwrap_or(&0)) + acc);

    println!("Part 2 ans: {ans}");

    Ok(())
}
