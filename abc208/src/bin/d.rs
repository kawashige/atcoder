use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        abc: [(usize, usize, usize); m]
    }

    let mut d = vec![vec![std::usize::MAX; n]; n];

    for i in 0..n {
        d[i][i] = 0;
    }

    for (a, b, c) in abc {
        d[a - 1][b - 1] = c;
    }

    let mut sum = 0;
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if d[i][k] != std::usize::MAX
                    && d[k][j] != std::usize::MAX
                    && d[i][j] > d[i][k] + d[k][j]
                {
                    d[i][j] = d[i][k] + d[k][j];
                }
            }
        }

        for i in 0..n {
            for j in 0..n {
                if d[i][j] != std::usize::MAX {
                    sum += d[i][j];
                }
            }
        }
    }

    println!("{}", sum);
}
