use std::io::Read;

use itertools::Itertools;

fn get_combo(op: u8, regs: &[i64]) -> i64 {
    match op {
        0..=3 => op as i64,
        4 => regs[0],
        5 => regs[1],
        6 => regs[2],
        _ => panic!("invalid operand"),
    }
}

fn main() -> anyhow::Result<()> {
    let filepath = "tests/17/1.txt";
    let mut contents = String::new();
    let _ = std::fs::File::open(filepath)?.read_to_string(&mut contents)?;
    let v = contents
        .split('\n')
        .filter_map(|s| {
            if s.is_empty() {
                None
            } else {
                Some(s.split_once(':').unwrap().1.trim_ascii())
            }
        })
        .collect::<Vec<_>>();

    let mut regs: Vec<i64> = v[0..3].iter().map(|s| s.parse().unwrap()).collect();
    let prog: Vec<u8> = v[3].split(',').map(|s| s.parse().unwrap()).collect();

    let mut i = 0;
    let mut out = vec![];

    while i < prog.len() {
        let (opcode, operand) = (prog[i], prog[i + 1]);

        println!("{opcode} {operand} {i} {regs:?}");

        match opcode {
            0 => {
                let nm = regs[0];
                let dm = 1 << get_combo(operand, &regs);
                regs[0] = nm / dm;
                i += 2;
            }
            1 => {
                regs[1] = regs[1] ^ operand as i64;
                i += 2;
            }
            2 => {
                regs[1] = get_combo(operand, &regs) % 8;
                i += 2;
            }
            3 => {
                if regs[0] != 0 {
                    i = operand as usize;
                } else {
                    i += 2;
                }
            }
            4 => {
                regs[1] = regs[1] ^ regs[2];
                i += 2;
            }
            5 => {
                out.push(get_combo(operand, &regs) % 8);
                i += 2;
            }
            6 => {
                let nm = regs[0];
                let dm = 1 << get_combo(operand, &regs);
                regs[1] = nm / dm;
                i += 2;
            }
            7 => {
                let nm = regs[0];
                let dm = 1 << get_combo(operand, &regs);
                regs[2] = nm / dm;
                i += 2;
            }
            _ => panic!("Invalid"),
        };
    }
    println!("{regs:?}");

    let ans = out.iter().map(|c| c.to_string()).join(",");
    println!("{ans}");

    Ok(())
}
