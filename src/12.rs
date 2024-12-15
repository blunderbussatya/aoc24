use std::{collections::HashSet, io::Read};
const DIRS: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

fn dfs(
    i: usize,
    j: usize,
    mat: &[Vec<char>],
    vis: &mut HashSet<u32>,
) -> (u64, u64, Vec<(usize, usize)>) {
    let (n, m) = (mat.len() as i32, mat[0].len() as i32);
    let cur = mat[i][j];
    let get_hash = |x, y| -> u32 { ((x as u32) * (m as u32)) + y as u32 };
    let fp = get_hash(i, j);
    if vis.contains(&fp) {
        return (0, 0, vec![]);
    }
    vis.insert(fp);
    let mut pr = 0;
    let mut ar = 1;
    let mut is_corner = false;
    let mut corners = vec![];

    for (x, y) in DIRS {
        let nx = i as i32 + x;
        let ny = j as i32 + y;

        let mut should_go = true;

        if nx < 0 || ny < 0 || nx >= n || ny >= m || mat[nx as usize][ny as usize] != cur {
            pr += 1;
            should_go = false;
            is_corner = true;
        }

        if !should_go || vis.contains(&get_hash(nx as usize, ny as usize)) {
            continue;
        }

        let (npr, nar, mut nac) = dfs(nx as usize, ny as usize, mat, vis);
        ar += nar;
        pr += npr;
        corners.append(&mut nac);
    }
    if is_corner {
        corners.push((i, j));
    }
    (pr, ar, corners)
}

fn director(idx: usize) -> usize {
    match idx {
        0 | 1 => 2,
        _ => 0,
    }
}

fn get_lines(pts: &[(usize, usize)]) -> u64 {
    if pts.is_empty() {
        return 0;
    }
    let (mut i, mut j) = (0, 0);

    let pts = pts
        .iter()
        .map(|(x, y)| (*x as i32, *y as i32))
        .collect::<HashSet<_>>();
    let mut vis = HashSet::new();
    let mut ans = 0;
    let mut idx = 0;

    loop {
        idx = director(idx);
        let mut smth_happened = false;
        for e in 0..4 {
            let nidx = (idx + e) % DIRS.len();
            let (dx, dy) = DIRS[nidx];
            let (nx, ny) = (i + dx, j + dy);
            if vis.contains(&(nx, ny, idx)) || !pts.contains(&(nx, ny)) {
                continue;
            } else {
                smth_happened = false;
                ans += 1;
                idx = nidx;
                println!("---ll {i} {j} {idx}");
                while !vis.contains(&(i, j, idx)) {
                    vis.insert((i, j, idx));
                    let (nx, ny) = (i + dx, j + dy);
                    if !pts.contains(&(nx, ny)) {
                        break;
                    }
                    (i, j) = (nx, ny);
                    println!("cc {i} {j} ");
                }
            }
        }
        if !smth_happened {
            break;
        }
        println!("--- {i} {j} {idx}");
    }
    println!("{vis:?}");

    ans
}

fn main() -> anyhow::Result<()> {
    let filepath = "tests/12/example.txt";
    let mut contents = String::new();
    let _ = std::fs::File::open(filepath)?.read_to_string(&mut contents)?;
    let mat = contents
        .split('\n')
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut ans1 = 0;
    let mut ans2 = 0;

    let mut vis = HashSet::new();
    let (n, m) = (mat.len(), mat[0].len());

    for (i, row) in mat.iter().enumerate() {
        for (j, ele) in row.iter().enumerate() {
            let (pr, ar, cr) = dfs(i, j, &mat, &mut vis);
            if ar != 0 {
                println!("cur {i} {j} {ele}");
            }
            let sides = get_lines(&cr);
            if ar != 0 {
                println!("side {sides}");
            }
            ans1 += pr * ar;
            ans2 += sides * ar;
            break;
        }
        break;
    }
    // println!("Part 1 {ans1}");
    println!("Part 2 {ans2}");

    Ok(())
}
