use std::io::Read;

const DIRS: [(i32, i32); 4] = [(0, 1), (1, 0), (1, 1), (1, -1)];

fn dfs(
    x: i32,
    y: i32,
    d: usize,
    i: usize,
    pat: &Vec<char>,
    mat: &Vec<Vec<char>>,
    // can use a hashmap but I want to reminisce my good old global array based CP days
    dp: &mut Vec<Vec<Vec<Vec<Option<i64>>>>>,
) -> i64 {
    if i == pat.len() {
        return 1;
    }
    if x < 0 || y < 0 || x as usize >= mat.len() || y as usize >= mat[0].len() {
        return 0;
    }
    if let Some(cached) = dp[x as usize][y as usize][d][i] {
        return cached;
    }
    let ans = match d {
        0 => {
            let mut cur = dfs(x + 1, y, 0, 0, pat, mat, dp) + dfs(x, y + 1, 0, 0, pat, mat, dp)
                - dfs(x + 1, y + 1, 0, 0, pat, mat, dp);
            cur += DIRS
                .iter()
                .enumerate()
                .filter_map(|(nd, (xx, yy))| {
                    if mat[x as usize][y as usize] == pat[i] {
                        Some(dfs(x + xx, y + yy, nd + 1, i + 1, pat, mat, dp))
                    } else {
                        None
                    }
                })
                .sum::<i64>();
            cur
        }
        dd => {
            if mat[x as usize][y as usize] != pat[i] {
                0
            } else {
                let (xx, yy) = DIRS[dd - 1];
                dfs(x + xx, y + yy, d, i + 1, pat, mat, dp)
            }
        }
    };
    dp[x as usize][y as usize][d][i] = Some(ans);
    ans
}

fn conv2d(mat: &Vec<Vec<char>>, mask: &Vec<Vec<Option<char>>>) -> u64 {
    let (mm, mn) = (mask.len(), mask[0].len());
    let (m, n) = (mat.len(), mat[0].len());
    let mut ans = 0;

    for i in 0..(m - mm + 1) {
        for j in 0..(n - mn + 1) {
            let mut mask_matched = true;
            for a in 0..mm {
                for b in 0..mn {
                    if mask[a][b].is_none() {
                        continue;
                    }
                    if mask[a][b].unwrap() != mat[i + a][j + b] {
                        mask_matched = false;
                        break;
                    }
                }
                if !mask_matched {
                    break;
                }
            }
            ans += mask_matched as u64;
        }
    }
    ans
}

fn mask_str_to_mask(mask: &str) -> Vec<Vec<Option<char>>> {
    mask.trim_ascii()
        .split("\n")
        .map(|s| {
            s.trim_ascii()
                .chars()
                .map(|c| match c {
                    '.' => None,
                    ch => Some(ch),
                })
                .collect()
        })
        .collect()
}

fn main() -> anyhow::Result<()> {
    let filepath = "tests/4/1.txt";
    let mut contents = String::new();
    let _ = std::fs::File::open(filepath)?.read_to_string(&mut contents)?;
    let mat = contents
        .split("\n")
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut pat = "XMAS".chars().collect::<Vec<_>>();
    let (n, m, p, d) = (mat.len(), mat[0].len(), pat.len(), DIRS.len() + 1);

    let mut ans = dfs(
        0,
        0,
        0,
        0,
        &pat,
        &mat,
        &mut vec![vec![vec![vec![None; p]; d]; m]; n],
    );
    pat.reverse();
    ans += dfs(
        0,
        0,
        0,
        0,
        &pat,
        &mat,
        &mut vec![vec![vec![vec![None; p]; d]; m]; n],
    );

    println!("Part 1 ans: {ans:#?}");

    let mask_strs = vec![
        "   
        M.S
        .A.
        M.S
        ",
        "
        M.M
        .A.
        S.S
        ",
        "
        S.M
        .A.
        S.M
        ",
        "
        S.S
        .A.
        M.M
        ",
    ];
    let masks = mask_strs
        .into_iter()
        .map(mask_str_to_mask)
        .collect::<Vec<_>>();
    let ans = masks.iter().map(|m| conv2d(&mat, m)).sum::<u64>();
    println!("Part 2 ans: {ans:#?}");
    Ok(())
}
