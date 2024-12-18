use std::{
    collections::{HashMap, HashSet, VecDeque},
    io::Read,
};

const DIRS: [(i64, i64); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

fn simulate(pts: &[(i64, i64)], (n, m): (i64, i64), take: usize) -> Option<i32> {
    let pts = pts.iter().cloned().take(take).collect::<HashSet<_>>();
    let mut q = VecDeque::new();
    q.push_back((0, 0));
    let mut vis = HashMap::new();
    vis.insert((0, 0), 0);
    while let Some((x, y)) = q.pop_front() {
        let d = vis[&(x, y)];
        for (dx, dy) in DIRS {
            let nx = x + dx;
            let ny = y + dy;
            let nxy = (nx, ny);
            if nx < 0
                || ny < 0
                || nx >= n
                || ny >= m
                || pts.contains(&nxy)
                || *vis.get(&nxy).unwrap_or(&i32::MAX) <= (d + 1)
            {
                continue;
            }
            vis.insert(nxy, d + 1);
            q.push_back((nx, ny));
        }
    }
    vis.get(&(n - 1, m - 1)).copied()
}

fn main() -> anyhow::Result<()> {
    let filepath = "tests/18/1.txt";
    let mut contents = String::new();
    let _ = std::fs::File::open(filepath)?.read_to_string(&mut contents)?;
    let pts: Vec<(i64, i64)> = contents
        .split('\n')
        .map(|c| {
            let (x1, x2) = c.split_once(',').unwrap();
            (x1.parse().unwrap(), x2.parse().unwrap())
        })
        .collect();

    let ans = simulate(&pts, (71, 71), 1024).unwrap();
    println!("Part 1 ans: {ans}");
    let mut start = 0;
    let mut end = pts.len() - 1;
    let mut idx = end;

    while start <= end {
        let mid = (start + end) / 2;
        let s = simulate(&pts, (71, 71), mid);
        if s.is_some() {
            start = mid + 1;
        } else {
            idx = mid;
            end = mid - 1;
        }
    }

    let ans = pts[idx - 1];
    println!("Part 2 ans: {},{}", ans.0, ans.1);

    Ok(())
}
