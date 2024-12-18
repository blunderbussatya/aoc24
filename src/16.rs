use std::{
    collections::{BTreeSet, HashMap, HashSet},
    io::Read,
};
const DIRS: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

fn dijkstra(mat: &[Vec<char>], start: (usize, usize, usize)) -> HashMap<(usize, usize), i64> {
    let (n, m) = (mat.len(), mat[0].len());
    let mut q = BTreeSet::new();
    q.insert((0, start.0, start.1, start.2));
    let mut vis = HashMap::new();
    vis.insert((start.0, start.1), 0);
    while let Some((cost, x, y, d)) = q.pop_first() {
        for (i, (dx, dy)) in DIRS.iter().enumerate() {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            let nc: i64 = cost + {
                if i == d {
                    1
                } else {
                    1001
                }
            };
            if nx < 0 || ny < 0 || nx >= n as i32 || ny >= m as i32 {
                continue;
            }
            let (nx, ny) = (nx as usize, ny as usize);
            let nxy = (nx, ny);
            if mat[nx][ny] == '#' || *vis.get(&nxy).unwrap_or(&i64::MAX) < nc {
                continue;
            }
            vis.insert(nxy, nc);
            q.insert((nc, nx, ny, i));
        }
    }
    vis
}

fn main() -> anyhow::Result<()> {
    let filepath = "tests/16/1.txt";
    let mut contents = String::new();
    let _ = std::fs::File::open(filepath)?.read_to_string(&mut contents)?;
    let mat = contents
        .split('\n')
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let (mut start, mut end) = (None, None);

    for (i, row) in mat.iter().enumerate() {
        for (j, ele) in row.iter().enumerate() {
            if *ele == 'S' {
                start = Some((i, j));
            } else if *ele == 'E' {
                end = Some((i, j));
            }
        }
    }

    let (start, end) = (start.unwrap(), end.unwrap());
    let se = dijkstra(&mat, (start.0, start.1, 0));

    let ans = se[&end];
    println!("Part 1 ans: {ans}");

    let ess = (0..4)
        .map(|i| dijkstra(&mat, (end.0, end.1, i)))
        .collect::<Vec<_>>();
    let (n, m) = (mat.len(), mat[0].len());
    let mut inpath = HashSet::new();
    for i in 0..n {
        for j in 0..m {
            let px = (i, j);
            if let Some(d1) = se.get(&px) {
                for es in ess.iter() {
                    if let Some(d2) = es.get(&px) {
                        if ans == (d1 + d2) {
                            inpath.insert((i, j));
                            break;
                        }
                    }
                }
            }
        }
    }
    let ans = inpath.len();

    println!("Part 2 ans {ans}");

    Ok(())
}
