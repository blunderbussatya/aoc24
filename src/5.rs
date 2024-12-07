use std::{
    collections::{HashMap, HashSet, VecDeque},
    io::Read,
};

fn get_topo_ordering(g: HashMap<i64, HashSet<i64>>) -> (HashMap<i64, i64>, Vec<i64>) {
    let mut indeg = g.values().flatten().fold(HashMap::new(), |mut map, v| {
        *map.entry(*v).or_insert(0) += 1;
        map
    });

    let mut q = VecDeque::new();

    g.keys().for_each(|v| {
        let cur = indeg.entry(*v).or_insert(0);
        if 0 == *cur {
            q.push_back(*v);
        }
    });

    let mut order = 0;
    let mut ordering = HashMap::new();
    let mut order_vec = vec![];

    while !q.is_empty() {
        let mut nx_q = VecDeque::new();
        while let Some(node) = q.pop_front() {
            ordering.insert(node, order);
            order_vec.push(node);
            if let Some(neighbors) = g.get(&node) {
                for ne in neighbors {
                    let ne_indeg = indeg.get_mut(ne).unwrap();
                    *ne_indeg -= 1;
                    if 0 == *ne_indeg {
                        nx_q.push_back(*ne);
                    }
                }
            }
        }
        q = nx_q;
        order += 1;
    }

    (ordering, order_vec)
}

fn create_graph(edges: &Vec<(i64, i64)>) -> HashMap<i64, HashSet<i64>> {
    edges.iter().fold(HashMap::new(), |mut g, (u, v)| {
        g.entry(*u).or_insert(HashSet::new()).insert(*v);
        g
    })
}

fn main() -> anyhow::Result<()> {
    let filepath = "tests/5/1.txt";
    let mut contents = String::new();
    let _ = std::fs::File::open(filepath)?.read_to_string(&mut contents)?;

    let (order, queries) = contents.split_once("\n\n").unwrap();

    let edges = order
        .split('\n')
        .map(|o| {
            let (a, b) = o.split_once('|').unwrap();
            (a.parse().unwrap(), b.parse().unwrap())
        })
        .collect();

    let g = create_graph(&edges);

    let queries: Vec<Vec<i64>> = queries
        .split('\n')
        .map(|s| s.split(',').map(|nm| nm.parse().unwrap()).collect())
        .collect();

    let (oq, nq): (Vec<_>, Vec<_>) = queries.into_iter().partition(|v| {
        let mut ok = true;
        for (i, val) in v.into_iter().enumerate() {
            for j in 0..i {
                if let Some(gx) = g.get(val) {
                    if gx.contains(&v[j]) {
                        ok = false;
                        break;
                    }
                }
            }
            if !ok {
                break;
            }
        }
        ok
    });

    let ans = oq.iter().map(|v| v[v.len() / 2]).sum::<i64>();

    println!("Part 1 {ans}");

    let ans = nq
        .into_iter()
        .map(|v| {
            let hv = v.into_iter().collect::<HashSet<_>>();
            let cur_edges = edges
                .iter()
                .filter(|(u, v)| hv.contains(u) && hv.contains(v))
                .cloned()
                .collect::<Vec<_>>();
            let g = create_graph(&cur_edges);
            let (_, cor) = get_topo_ordering(g);
            cor[cor.len() / 2]
        })
        .sum::<i64>();

    println!("Part 2 {ans}");

    Ok(())
}
