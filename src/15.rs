use std::io::Read;

use itertools::Itertools;

fn get_cmd(c: char) -> (i32, i32) {
    match c {
        '>' => (0, 1),
        '<' => (0, -1),
        'v' => (1, 0),
        '^' => (-1, 0),
        _ => panic!("invalid cmd"),
    }
}

fn shift(mat: &mut Vec<Vec<char>>, (mut x, mut y): (i32, i32), c: char) -> Option<(usize, usize)> {
    let (dx, dy) = get_cmd(c);
    let (ox, oy) = (x, y);
    let (n, m) = (mat.len() as i32, mat[0].len() as i32);
    let mut cnt = 0;
    let found_dot = loop {
        cnt += 1;
        let (nx, ny) = (x + dx, y + dy);
        if nx < 0 || ny < 0 || nx >= n || ny >= m {
            break false;
        }
        (x, y) = (nx, ny);
        if mat[x as usize][y as usize] == '#' {
            break false;
        }
        if mat[x as usize][y as usize] == 'O' {
            continue;
        }
        if mat[x as usize][y as usize] == '.' {
            break true;
        }
    };
    if found_dot {
        let (mut ix, mut iy) = (ox, oy);
        for _ in 0..cnt {
            let (nx, ny) = (ix + dx, iy + dy);
            mat[nx as usize][ny as usize] = 'O';
            (ix, iy) = (nx, ny);
        }
        mat[ox as usize][oy as usize] = '.';
        let (sx, sy) = ((ox + dx) as usize, (oy + dy) as usize);
        mat[sx][sy] = '@';
        Some((sx, sy))
    } else {
        None
    }
}

#[allow(dead_code)]
fn pr(mat: &Vec<Vec<char>>) {
    for row in mat.iter() {
        for ele in row.iter() {
            print!("{ele}");
        }
        println!();
    }
}

fn main() -> anyhow::Result<()> {
    let filepath = "tests/15/1.txt";
    let mut contents = String::new();
    let _ = std::fs::File::open(filepath)?.read_to_string(&mut contents)?;
    let (mat, cmds) = contents.split_once("\n\n").unwrap();

    let mut mat = mat
        .split('\n')
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let cmds = cmds.split('\n').join("");

    let (mut sx, mut sy) = (None, None);
    for (i, row) in mat.iter().enumerate() {
        for (j, ele) in row.iter().enumerate() {
            if *ele == '@' {
                (sx, sy) = (Some(i), Some(j));
            }
        }
    }

    let (mut x, mut y) = (sx.unwrap(), sy.unwrap());
    for c in cmds.chars() {
        if let Some(nxy) = shift(&mut mat, (x as i32, y as i32), c) {
            (x, y) = nxy;
        }
    }

    let mut ans = 0;
    for (i, row) in mat.iter().enumerate() {
        for (j, ele) in row.iter().enumerate() {
            if *ele == 'O' {
                ans += i * 100 + j;
            }
        }
    }
    println!("Part 1 ans: {ans}");

    Ok(())
}
