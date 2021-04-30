use proconio::input;

fn main() {
    input! {
        n: u64,
        b: usize,
        k: usize,
        c: [usize; k]
    }

    let mut dp = vec![vec![0; b]; 2];
    dp[1][0] = 1;
    let mut i = 0;

    for _ in 0..n {
        for l in 0..b {
            for m in 0..k {
                let remains = (l * 10 + c[m]) % b;
                dp[i][remains] += dp[(i + 1) % 2][l];
                dp[i][remains] %= 1_000_000_007;
            }
        }
        i = (i + 1) % 2;
        dp[i] = vec![0; b];
    }

    println!("{}", dp[(i + 1) % 2][0]);
}
