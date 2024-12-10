use std::{
    collections::{HashMap, HashSet},
    io::Read,
};

const DIRS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn dfs1(i: usize, j: usize, mat: &[Vec<u32>], dp: &mut HashMap<u32, HashSet<u32>>) -> HashSet<u32> {
    let (n, m) = (mat.len() as i32, mat[0].len() as i32);
    let cur = mat[i][j];
    let fp = ((i as u32) * (n as u32)) + j as u32;
    if let Some(cv) = dp.get(&fp) {
        return cv.clone();
    }
    if cur == 9 {
        let mut ret = HashSet::new();
        ret.insert(fp);
        dp.insert(fp, ret.clone());
        return ret;
    }
    let ans = DIRS
        .iter()
        .flat_map(|(x, y)| {
            let nx = i as i32 + x;
            let ny = j as i32 + y;
            if nx >= 0 && ny >= 0 && nx < n && ny < m && mat[nx as usize][ny as usize] == (cur + 1)
            {
                dfs1(nx as usize, ny as usize, mat, dp)
            } else {
                HashSet::new()
            }
        })
        .collect::<HashSet<_>>();
    dp.insert(fp, ans.clone());
    ans
}

fn dfs2(i: usize, j: usize, mat: &[Vec<u32>], dp: &mut HashMap<u32, u64>) -> u64 {
    let (n, m) = (mat.len() as i32, mat[0].len() as i32);
    let cur = mat[i][j];
    if cur == 9 {
        return 1;
    }
    let fp = ((i as u32) * (n as u32)) + j as u32;
    if let Some(cv) = dp.get(&fp) {
        return *cv;
    }
    let ans = DIRS
        .iter()
        .map(|(x, y)| {
            let nx = i as i32 + x;
            let ny = j as i32 + y;
            if nx >= 0 && ny >= 0 && nx < n && ny < m && mat[nx as usize][ny as usize] == (cur + 1)
            {
                dfs2(nx as usize, ny as usize, mat, dp)
            } else {
                0
            }
        })
        .sum();
    dp.insert(fp, ans);
    ans
}

fn main() -> anyhow::Result<()> {
    let filepath = "tests/10/1.txt";
    let mut contents = String::new();
    let _ = std::fs::File::open(filepath)?.read_to_string(&mut contents)?;
    let mat: Vec<Vec<u32>> = contents
        .split('\n')
        .map(|s| s.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let mut ans = 0;
    let mut dp = HashMap::new();
    for (i, row) in mat.iter().enumerate() {
        for (j, ele) in row.iter().enumerate() {
            if *ele == 0 {
                ans += dfs1(i, j, &mat, &mut dp).len();
            }
        }
    }
    println!("Part 1 {ans}");

    let mut ans = 0;
    let mut dp = HashMap::new();
    for (i, row) in mat.iter().enumerate() {
        for (j, ele) in row.iter().enumerate() {
            if *ele == 0 {
                ans += dfs2(i, j, &mat, &mut dp);
            }
        }
    }
    println!("Part 2 {ans}");

    Ok(())
}
