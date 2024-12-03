use regex::Regex;
use std::io::Read;

fn get_muls(input: &str) -> anyhow::Result<i64> {
    let mul_pattern = r"mul\((\d{1,3}),(\d{1,3})\)";
    let regex = Regex::new(mul_pattern)?;
    let ans = regex
        .captures_iter(input)
        .map(|c| (c[1].parse::<i64>().unwrap(), c[2].parse::<i64>().unwrap()))
        .fold(0, |acc, (n1, n2)| acc + n1 * n2);
    Ok(ans)
}

fn main() -> anyhow::Result<()> {
    let filepath = "tests/3/1.txt";
    let mut contents = String::new();
    let _ = std::fs::File::open(filepath)?.read_to_string(&mut contents)?;
    let ans = get_muls(&contents)?;
    println!("Part 1 ans: {ans}");
    let ans = contents.split("do()").fold(0, |acc, dos| {
        acc + get_muls(dos.split("don't()").next().unwrap()).unwrap()
    });
    println!("Part 2 ans: {ans}");
    Ok(())
}
