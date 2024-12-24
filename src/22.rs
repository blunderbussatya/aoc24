use std::io::Read;

const MOD: i64 = 16777216;

fn mix(n: i64, m: i64) -> i64 {
    n ^ m
}

fn prune(n: i64) -> i64 {
    n % MOD
}

fn process(n: i64) -> i64 {
    let nx = prune(mix(n * 64, n));
    let nx = prune(mix(nx / 32, nx));
    prune(mix(nx * 2048, nx))
}

fn rep_process(n: i64, times: usize) -> i64 {
    (0..times).fold(n, |acc, _| process(acc))
}

fn main() -> anyhow::Result<()> {
    let filepath = "tests/22/1.txt";
    let mut contents = String::new();
    let _ = std::fs::File::open(filepath)?.read_to_string(&mut contents)?;
    let nms: Vec<i64> = contents.split('\n').map(|x| x.parse().unwrap()).collect();
    let rep_2k = |n| rep_process(n, 2000);

    let ans = nms.into_iter().map(rep_2k).sum::<i64>();

    println!("{ans}");

    Ok(())
}
