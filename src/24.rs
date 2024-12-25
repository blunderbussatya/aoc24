use std::{
    collections::{HashMap, HashSet},
    io::Read,
};

use itertools::Itertools;

fn operate(a: i32, op: &str, b: i32) -> i32 {
    match op {
        "AND" => a & b,
        "OR" => a | b,
        "XOR" => a ^ b,
        _ => panic!("Invalid cmd"),
    }
}

fn main() -> anyhow::Result<()> {
    let filepath = "tests/24/1.txt";
    let mut contents = String::new();
    let _ = std::fs::File::open(filepath)?.read_to_string(&mut contents)?;
    let (ix, gx) = contents.split_once("\n\n").unwrap();
    let mut vals: HashMap<_, i32> = ix
        .split('\n')
        .map(|c| {
            let (n, i) = c.split_once(": ").unwrap();
            (n, i.parse().unwrap())
        })
        .collect();

    let mut rels = gx
        .split('\n')
        .map(|c| {
            let cx = c.split(' ').collect::<Vec<_>>();
            (cx[0], cx[1], cx[2], cx[4])
        })
        .collect::<HashSet<_>>();

    while !rels.is_empty() {
        let cur = rels
            .iter()
            .find(|r| vals.contains_key(r.0) && vals.contains_key(r.2))
            .unwrap()
            .clone();

        let (op1, op, op2, op3) = &cur;
        let vx = operate(vals[op1], op, vals[op2]);
        vals.insert(op3, vx);
        rels.remove(&cur);
    }
    let zs = vals
        .into_iter()
        .filter(|(k, _)| k.starts_with('z'))
        .sorted()
        .map(|(_, v)| v)
        .collect_vec();

    let ans = zs
        .into_iter()
        .enumerate()
        .fold(0, |acc, (idx, bit)| acc + (1i64 << idx) * (*bit as i64));

    println!("{ans}");

    Ok(())
}
