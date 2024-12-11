use std::{collections::HashMap, io::Read};

fn rp(v: Vec<i128>, r: u32) -> usize {
    let mut m = v.into_iter().fold(HashMap::new(), |mut map, val| {
        *map.entry(val).or_insert(0) += 1;
        map
    });
    for _ in 0..r {
        m = m.into_iter().fold(HashMap::new(), |mut map, (x, f)| {
            let s = x.to_string();
            if x == 0 {
                *map.entry(1).or_insert(0) += f;
            } else if s.len() % 2 == 0 {
                let v1 = s[..s.len() / 2].parse().unwrap();
                let v2 = s[s.len() / 2..].parse().unwrap();
                *map.entry(v1).or_insert(0) += f;
                *map.entry(v2).or_insert(0) += f;
            } else {
                *map.entry(x * 2024).or_insert(0) += f;
            }
            map
        })
    }
    m.values().sum()
}

fn main() -> anyhow::Result<()> {
    let filepath = "tests/11/1.txt";
    let mut contents = String::new();
    let _ = std::fs::File::open(filepath)?.read_to_string(&mut contents)?;
    let v: Vec<i128> = contents.split(' ').map(|s| s.parse().unwrap()).collect();
    let ans = rp(v.clone(), 25);
    println!("Part 1 {ans}");
    let ans = rp(v, 75);
    println!("Part 2 {ans}");

    Ok(())
}
