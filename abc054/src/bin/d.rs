use proconio::input;

fn main() {
    input! {
        n: usize,
        ma: usize,
        mb: usize,
        abc: [(usize, usize, usize); n]
    }

    let l = n * 10 + 1;
    let mut dp = vec![vec![std::usize::MAX; l]; l];
    dp[0][0] = 0;
    let mut r = std::usize::MAX;

    for i in 0..n {
        for j in (0..l).rev() {
            for k in (0..l).rev() {
                if dp[j][k] != std::usize::MAX {
                    dp[j + abc[i].0][k + abc[i].1] =
                        std::cmp::min(dp[j + abc[i].0][k + abc[i].1], dp[j][k] + abc[i].2);
                    if mb * (j + abc[i].0) == ma * (k + abc[i].1) {
                        r = std::cmp::min(r, dp[j + abc[i].0][k + abc[i].1]);
                    }
                }
            }
        }
    }

    if r == std::usize::MAX {
        println!("-1");
    } else {
        println!("{}", r);
    }
}
