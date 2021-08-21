use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        abt: [(usize, usize, usize); m]
    }

    let mut dist = vec![vec![std::usize::MAX; n]; n];
    for i in 0..n {
        dist[i][i] = 0;
    }

    for (a, b, t) in abt {
        dist[a - 1][b - 1] = t;
        dist[b - 1][a - 1] = t;
    }

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if dist[i][k] != std::usize::MAX
                    && dist[k][j] != std::usize::MAX
                    && dist[i][j] > dist[i][k] + dist[k][j]
                {
                    dist[i][j] = dist[i][k] + dist[k][j]
                }
            }
        }
    }

    println!(
        "{}",
        dist.into_iter()
            .map(|v| v.into_iter().max().unwrap())
            .min()
            .unwrap()
    );
}
