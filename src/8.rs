use std::{
    collections::{HashMap, HashSet},
    io::Read,
};

fn main() -> anyhow::Result<()> {
    let filepath = "tests/8/1.txt";
    let mut contents = String::new();
    let _ = std::fs::File::open(filepath)?.read_to_string(&mut contents)?;
    let mat = contents
        .split('\n')
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut mp: HashMap<char, Vec<_>> = HashMap::new();

    for (i, row) in mat.iter().enumerate() {
        for (j, ele) in row.iter().enumerate() {
            if *ele == '.' {
                continue;
            }
            mp.entry(*ele).or_default().push((i as i32, j as i32));
        }
    }

    let (n, m) = (mat.len() as i32, mat[0].len() as i32);
    let is_okay = |px| -> bool {
        let (x, y) = px;
        if x < 0 || y < 0 || x >= n || y >= m {
            return false;
        }
        true
    };

    let ans = mp
        .values()
        .flat_map(|v| {
            let mut vx = vec![];
            for i in 0..v.len() {
                for j in (i + 1)..v.len() {
                    let (x1, y1) = v[i];
                    let (x2, y2) = v[j];
                    let (dx, dy) = (x2 - x1, y2 - y1);
                    let p1 = (x1 - dx, y1 - dy);
                    let p2 = (x2 + dx, y2 + dy);
                    [p1, p2].into_iter().for_each(|p| {
                        if is_okay(p) {
                            vx.push(p);
                        }
                    });
                }
            }
            vx
        })
        .collect::<HashSet<_>>()
        .len();

    println!("Part 1 {ans}");

    let ans = mp
        .values()
        .flat_map(|v| {
            let mut vx = vec![];
            for i in 0..v.len() {
                for j in (i + 1)..v.len() {
                    let (x1, y1) = v[i];
                    let (x2, y2) = v[j];
                    let (dx, dy) = (x2 - x1, y2 - y1);
                    vx.push((x1, y1));
                    let mut c = (x1, y1);
                    loop {
                        c = (c.0 - dx, c.1 - dy);
                        if is_okay(c) {
                            vx.push(c);
                        } else {
                            break;
                        }
                    }
                    let mut c = (x1, y1);
                    loop {
                        c = (c.0 + dx, c.1 + dy);
                        if is_okay(c) {
                            vx.push(c);
                        } else {
                            break;
                        }
                    }
                }
            }
            vx
        })
        .collect::<HashSet<_>>()
        .len();

    println!("Part 2 {ans}");

    Ok(())
}
