use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        k: usize
    }

    let m = 1_000_000_007;

    let mut counts = vec![0; 8];
    counts[0] = 1;
    counts[1] = 2;
    for i in 2..8 {
        let mut count = 0;
        for j in 0..2_usize.pow(i) {
            if (0..63).all(|k| j & 1 << k == 0 || j & 1 << (k + 1) == 0) {
                count += 1;
            }
        }
        counts[i as usize] = count;
    }

    let mut dp = vec![vec![0_u64; w]; h + 1];
    dp[0][0] = 1;

    for i in 1..=h {
        for j in 0..w {
            dp[i][j] = (dp[i - 1][j]
                * (if j < 1 { 1 } else { counts[j - 1] } % m)
                * if w - j < 2 { 1 } else { counts[w - j - 2] })
                % m;
            if j > 0 {
                dp[i][j] += (dp[i - 1][j - 1]
                    * (if j < 2 { 1 } else { counts[j - 2] } % m)
                    * if w - j < 2 { 1 } else { counts[w - j - 2] })
                    % m;
                dp[i][j] %= m;
            }
            if j < w - 1 {
                dp[i][j] += (dp[i - 1][j + 1]
                    * (if j < 1 { 1 } else { counts[j - 1] } % m)
                    * if w - j < 3 { 1 } else { counts[w - j - 3] })
                    % m;
                dp[i][j] %= m;
            }
        }
    }

    // println!("{:?}", dp);

    println!("{}", dp[h][k - 1]);
}
