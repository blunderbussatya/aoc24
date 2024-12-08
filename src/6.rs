use std::{collections::HashSet, io::Read};

const DIRS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn is_guard(ch: char) -> Option<usize> {
    match ch {
        '^' => Some(3),
        '>' => Some(0),
        'v' => Some(1),
        '<' => Some(2),
        _ => None,
    }
}

fn get_cycle(
    mat: &[Vec<char>],
    extra_pt: Option<(usize, usize)>,
    cur_pos: (usize, usize, usize),
) -> (usize, bool) {
    let (n, m) = (mat.len(), mat[0].len());
    let (mut dir, mut cur_x, mut cur_y) = cur_pos;
    let mut vis = HashSet::new();
    let mut ns = HashSet::new();

    loop {
        vis.insert((dir, cur_x, cur_y));
        ns.insert((cur_x, cur_y));
        let (xx, yy) = DIRS[dir];
        let (nx, ny) = (cur_x as i32 + xx, cur_y as i32 + yy);
        // we are out of bounds
        if nx < 0 || ny < 0 || nx >= n as i32 || ny >= m as i32 {
            break (ns.len(), false);
        }
        // we got a cycle
        if vis.contains(&(dir, nx as usize, ny as usize)) {
            break (ns.len(), true);
        }

        let (nx, ny) = (nx as usize, ny as usize);

        if mat[nx][ny] == '#' || (extra_pt.is_some() && extra_pt.unwrap() == (nx, ny)) {
            dir = (dir + 1) % 4;
        } else {
            cur_x = nx;
            cur_y = ny;
        }
    }
}

fn main() -> anyhow::Result<()> {
    let filepath = "tests/6/1.txt";
    let mut contents = String::new();
    let _ = std::fs::File::open(filepath)?.read_to_string(&mut contents)?;
    let mat = contents
        .split('\n')
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut g = None;
    for (i, row) in mat.iter().enumerate() {
        for (j, ele) in row.iter().enumerate() {
            if let Some(idx) = is_guard(*ele) {
                g = Some((idx, i, j));
                break;
            }
        }
        if g.is_some() {
            break;
        }
    }

    let (ans, _) = get_cycle(&mat, None, g.unwrap());
    println!("Part 1 {ans}");

    let mut ans = 0;
    // very brute force, run in release mode to get answer quickly
    for (i, row) in mat.iter().enumerate() {
        for (j, ele) in row.iter().enumerate() {
            if is_guard(*ele).is_some() || *ele == '#' {
                continue;
            } else {
                ans += get_cycle(&mat, Some((i, j)), g.unwrap()).1 as i32;
            }
        }
    }
    println!("Part 2 {ans}");
    Ok(())
}
