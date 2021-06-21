use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        abc: [(usize, usize, i64); m]
    }

    let mut list = vec![vec![]; n];
    for (a, b, c) in abc {
        list[a - 1].push((b - 1, -c));
    }

    let mut dist = vec![std::i64::MAX; n];
    dist[0] = 0;

    let mut n_score = 0;
    for i in 0..(2 * n) {
        for v in 0..n {
            for (next, distance) in &list[v] {
                if dist[v] != std::i64::MAX && dist[*next] > dist[v] + distance {
                    dist[*next] = dist[v] + distance;
                }
            }
        }
        if i == n - 1 {
            n_score = dist[n - 1];
        }
    }

    if n_score != dist[n - 1] {
        println!("inf");
    } else {
        println!("{:?}", -dist[n - 1]);
    }
}
