use std::{collections::HashSet, io::Read};

fn main() -> anyhow::Result<()> {
    let filepath = "tests/23/example.txt";
    let mut contents = String::new();
    let _ = std::fs::File::open(filepath)?.read_to_string(&mut contents)?;

    let con = contents
        .split('\n')
        .flat_map(|c| {
            let (c1, c2) = c.split_once('-').unwrap();
            [(c1, c2), (c2, c1)]
        })
        .collect::<HashSet<_>>();

    let comps = con.iter().map(|(s, _)| *s).collect::<HashSet<_>>();
    println!("{}", comps.len());

    let mut can = HashSet::new();

    for ts in comps.iter().filter(|s| s.starts_with('t')) {
        for a in comps.iter() {
            if ts == a {
                continue;
            }
            let con1 = (*ts, *a);
            for b in comps.iter() {
                if a == b || ts == b {
                    continue;
                }
                let con2 = (*ts, *b);
                let con3 = (*a, *b);
                if [con1, con2, con3].iter().all(|c| con.contains(c)) {
                    let mut so = vec![ts, a, b];
                    so.sort();
                    can.insert(so);
                }
            }
        }
    }

    let ans = can.len();
    println!("{ans}");

    Ok(())
}
