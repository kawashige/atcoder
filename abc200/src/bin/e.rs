use proconio::input;

fn main() {
    input! {
        n: usize,
        mut k: usize
    }

    let mut dp = vec![vec![0_i64; 3_000_005]; 4];
    dp[0][0] = 1;

    for i in 0..3 {
        for j in 0..=(i * n) {
            dp[i + 1][j + 1] += dp[i][j];
            dp[i + 1][j + n + 1] -= dp[i][j];
        }
        for j in 1..=((i + 1) * n) {
            dp[i + 1][j] += dp[i + 1][j - 1];
        }
    }

    let mut x = 0;
    for i in 0..=(3 * n) {
        if k <= dp[3][i] as usize {
            x = i;
            break;
        } else {
            k -= dp[3][i] as usize;
        }
    }

    for i in 1..=n {
        let min = if x <= i + n { 1 } else { x - i - n };
        let max = if x < i + 1 { 0 } else { n.min(x - i - 1) };
        if min > max {
            continue;
        }
        if k > max - min + 1 {
            k -= max - min + 1;
            continue;
        }
        let y = min + k - 1;
        let z = x - i - y;
        println!("{} {} {}", i, y, z);
        return;
    }
}
