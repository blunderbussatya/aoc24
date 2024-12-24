use std::io::Read;

fn main() -> anyhow::Result<()> {
    let filepath = "tests/16/1.txt";
    let mut contents = String::new();
    let _ = std::fs::File::open(filepath)?.read_to_string(&mut contents)?;
    let mat = contents
        .split('\n')
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let (n, m) = (mat.len(), mat[0].len());
    println!("{n} {m}");

    Ok(())
}
