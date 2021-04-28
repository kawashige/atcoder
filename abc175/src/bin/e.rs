use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        r: usize,
        c: usize,
        k: usize,
        rcv: [(usize, usize, u64); k]
    }

    let map = rcv.into_iter().fold(HashMap::new(), |mut map, (r, c, v)| {
        map.insert((r - 1, c - 1), v);
        map
    });

    let mut dp = vec![vec![vec![0; 4]; c]; 2];
    let mut index = 0;

    for i in 0..r {
        for j in 0..c {
            let c = *map.get(&(i, j)).unwrap_or(&0);

            dp[index][j][1] = c;
            if i > 0 {
                dp[index][j][0] = *dp[(index + 1) % 2][j].iter().max().unwrap();
                if c > 0 {
                    dp[index][j][1] = c + *dp[(index + 1) % 2][j].iter().max().unwrap();
                }
            }
            if j > 0 {
                if c == 0 {
                    dp[index][j][0] = std::cmp::max(dp[index][j][0], c + dp[index][j - 1][0]);
                    dp[index][j][1] = std::cmp::max(dp[index][j][1], c + dp[index][j - 1][1]);
                    dp[index][j][2] = c + dp[index][j - 1][2];
                    dp[index][j][3] = c + dp[index][j - 1][3];
                } else {
                    dp[index][j][0] = std::cmp::max(dp[index][j][0], dp[index][j - 1][0]);
                    dp[index][j][1] = std::cmp::max(
                        dp[index][j][1],
                        std::cmp::max(c + dp[index][j - 1][0], dp[index][j - 1][1]),
                    );
                    dp[index][j][2] = std::cmp::max(c + dp[index][j - 1][1], dp[index][j - 1][2]);
                    dp[index][j][3] = std::cmp::max(c + dp[index][j - 1][2], dp[index][j - 1][3]);
                }
            }
        }

        index = (index + 1) % 2;
        dp[index] = vec![vec![0; 4]; c];
    }

    let r = dp[(index + 1) % 2][c - 1].iter().max().unwrap();

    println!("{}", r);
}
