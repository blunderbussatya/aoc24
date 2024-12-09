use std::io::Read;

fn compress(content: &str) -> u64 {
    let mut m = content
        .chars()
        .enumerate()
        .flat_map(|(i, s)| {
            let n = s.to_digit(10).unwrap();
            let cur = {
                if i % 2 == 0 {
                    Some(i / 2)
                } else {
                    None
                }
            };
            vec![cur; n as usize]
        })
        .collect::<Vec<_>>();
    let mut i = 0;
    let mut j = m.len() - 1;
    while i < j {
        if m[i].is_some() {
            i += 1;
        } else if m[j].is_none() {
            j -= 1;
        } else {
            m[i] = m[j];
            m[j] = None;
        }
    }
    m.into_iter()
        .enumerate()
        .filter_map(|(i, c)| c.map(|v| (v * i) as u64))
        .sum()
}

#[derive(Debug, Clone)]
enum FS {
    Empty(u32),
    Occupied((u32, usize)),
}

fn compress2(content: &str) -> usize {
    let mut m = content
        .chars()
        .enumerate()
        .map(|(i, s)| {
            let n = s.to_digit(10).unwrap();
            if i % 2 == 0 {
                FS::Occupied((n, i / 2))
            } else {
                FS::Empty(n)
            }
        })
        .collect::<Vec<_>>();

    let mut i = 0;
    while i < m.len() {
        match m[i] {
            FS::Occupied(_) => {}
            FS::Empty(e) => {
                for j in ((i + 1)..m.len()).rev() {
                    if let FS::Occupied((ee, _)) = m[j] {
                        if ee > e {
                            continue;
                        }
                        let mj = m[j].clone();
                        m[j] = FS::Empty(ee);
                        if e == ee {
                            m[i] = mj;
                        } else {
                            m[i] = FS::Empty(e - ee);
                            m.insert(i, mj);
                        }
                        break;
                    }
                }
            }
        }
        i += 1;
    }

    m.into_iter()
        .flat_map(|x| match x {
            FS::Empty(e) => vec![0; e as usize],
            FS::Occupied((e, id)) => vec![id; e as usize],
        })
        .enumerate()
        .map(|(i, v)| i * v)
        .sum()
}

fn main() -> anyhow::Result<()> {
    let filepath = "tests/9/1.txt";
    let mut contents = String::new();
    let _ = std::fs::File::open(filepath)?.read_to_string(&mut contents)?;

    let ans = compress(&contents);
    println!("Part 1 {ans}");
    let ans = compress2(&contents);
    println!("Part 2 {ans}");
    Ok(())
}
