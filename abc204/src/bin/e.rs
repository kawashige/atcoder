use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut abcd: [[usize; 4]; m]
    }

    let mut list = vec![vec![]; n];

    let mut abcd = abcd
        .into_iter()
        .map(|v| {
            let mut t = (v[3] as f64).sqrt() as usize;
            while t * t + 1 <= v[3] {
                t += 1;
            }
            if t > 0 {
                t -= 1;
            }
            vec![v[0], v[1], v[2] + v[3] / (t + 1), v[2], v[3], t]
        })
        .collect::<Vec<Vec<usize>>>();
    abcd.sort_unstable();

    for i in 0..m {
        if abcd[i][0] == abcd[i][1] {
            continue;
        }
        list[abcd[i][0] - 1].push((abcd[i][1] - 1, abcd[i][3], abcd[i][4], abcd[i][5]));
        list[abcd[i][1] - 1].push((abcd[i][0] - 1, abcd[i][3], abcd[i][4], abcd[i][5]));
    }

    let mut heap = BinaryHeap::new();
    heap.push(Reverse((0 as u64, 0)));
    let mut seen = vec![false; n];

    while let Some(Reverse((time, node))) = heap.pop() {
        if node == n - 1 {
            println!("{}", time);
            return;
        }

        if seen[node] {
            continue;
        }

        seen[node] = true;

        for (next, c, d, t) in &list[node] {
            if seen[*next] {
                continue;
            }

            let tt = time.max(*t as u64);
            let next_time = tt + *c as u64 + *d as u64 / (tt + 1);
            heap.push(Reverse((next_time, *next)));
        }
    }

    println!("-1");
}
