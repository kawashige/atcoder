use proconio::input;

fn dfs(
    path: &mut Vec<usize>,
    r: &Vec<usize>,
    seen: &mut Vec<bool>,
    dist: &Vec<Vec<usize>>,
    d: usize,
    min: &mut usize,
) {
    if path.len() == r.len() {
        *min = std::cmp::min(*min, d);
        return;
    }

    if &d > min {
        return;
    }

    for i in 0..r.len() {
        if seen[i] {
            continue;
        }

        seen[i] = true;
        path.push(r[i] - 1);

        let new_d = d + if path.len() == 1 {
            0
        } else {
            dist[path[path.len() - 2]][path[path.len() - 1]]
        };

        dfs(path, r, seen, dist, new_d, min);

        seen[i] = false;
        path.pop();
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        r: usize,
        rs: [usize; r],
        abc: [(usize, usize, usize); m]
    }

    let mut dist = vec![vec![1_000_000_000; n]; n];
    for (a, b, c) in abc {
        dist[a - 1][b - 1] = c;
        dist[b - 1][a - 1] = c;
    }

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if dist[i][j] > dist[i][k] + dist[k][j] {
                    dist[i][j] = dist[i][k] + dist[k][j];
                }
            }
        }
    }

    let mut min = std::usize::MAX;

    dfs(
        &mut Vec::new(),
        &rs,
        &mut vec![false; r],
        &dist,
        0,
        &mut min,
    );

    println!("{}", min);
}
