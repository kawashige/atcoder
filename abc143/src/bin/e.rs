use proconio::fastout;
use proconio::input;
use std::{cmp::Reverse, collections::BinaryHeap};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        l: usize,
        abc: [(usize, usize, usize); m],
        q: usize,
        st: [(usize, usize); q]
    }

    let mut list = vec![vec![]; n];
    for (a, b, c) in abc {
        if c <= l {
            list[a - 1].push((b - 1, c));
            list[b - 1].push((a - 1, c));
        }
    }

    let mut count = vec![vec![std::usize::MAX; n]; n];
    let mut remains = vec![vec![0; n]; n];

    for i in 0..n {
        count[i][i] = 0;
        let mut heap = BinaryHeap::new();
        heap.push(Reverse((0, Reverse(l), i)));

        while let Some(Reverse((c, Reverse(r), v))) = heap.pop() {
            if count[i][v] < c || remains[i][v] > r {
                continue;
            }

            for (next, d) in &list[v] {
                let new_r = if &r >= d { r - d } else { l - d };
                let new_c = if &r >= d { 0 } else { 1 };
                if count[i][v] != std::usize::MAX
                    && (count[i][*next] > count[i][v] + new_c
                        || (count[i][*next] == count[i][v] + new_c && remains[i][*next] < new_r))
                {
                    count[i][*next] = count[i][v] + if &r >= d { 0 } else { 1 };
                    remains[i][*next] = new_r;

                    heap.push(Reverse((count[i][*next], Reverse(new_r), *next)));
                }
            }
        }
    }

    for (s, t) in st {
        if count[s - 1][t - 1] == std::usize::MAX {
            println!("-1");
        } else {
            println!("{}", count[s - 1][t - 1]);
        }
    }
}
