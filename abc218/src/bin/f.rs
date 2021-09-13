use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        st: [(usize, usize); m]
    }

    let mut g = vec![vec![-1; n]; n];
    for (i, (s, t)) in st.iter().enumerate() {
        g[s - 1][t - 1] = i as i32;
    }

    let mut queue = VecDeque::new();
    queue.push_back(0);
    let mut dist = vec![-1; n];
    dist[0] = 0;
    let mut memo = vec![(0, 0); n];
    while let Some(i) = queue.pop_front() {
        for j in 0..n {
            if dist[j] == -1 && g[i][j] != -1 {
                dist[j] = dist[i] + 1;
                memo[j] = (i, g[i][j]);
                queue.push_back(j);
            }
        }
    }

    if dist[n - 1] == -1 {
        for _ in 0..m {
            println!("-1");
        }
        return;
    }

    let mut shortest_path = Vec::new();
    let mut cur = n - 1;
    while cur != 0 {
        shortest_path.push(memo[cur].1);
        cur = memo[cur].0;
    }

    let mut r = vec![dist[n - 1]; m];
    for e in shortest_path {
        g[st[e as usize].0 - 1][st[e as usize].1 - 1] = -1;
        let mut queue = VecDeque::new();
        queue.push_back(0);
        let mut dist = vec![-1; n];
        dist[0] = 0;
        while let Some(i) = queue.pop_front() {
            for j in 0..n {
                if dist[j] == -1 && g[i][j] != -1 {
                    dist[j] = dist[i] + 1;
                    queue.push_back(j);
                }
            }
        }
        r[e as usize] = dist[n - 1];
        g[st[e as usize].0 - 1][st[e as usize].1 - 1] = e;
    }

    for x in r {
        println!("{}", x);
    }
}
