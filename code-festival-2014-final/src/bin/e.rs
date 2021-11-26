use proconio::input;

fn main() {
    input! {
        n: usize,
        r: [i32; n]
    }

    let mut dp = vec![vec![1; 2]; n];
    let mut count = 0;

    for i in 1..n {
        for j in 0..i {
            if r[j] < r[i] {
                dp[i][1] = dp[i][1].max(dp[j][0] + 1);
                count = count.max(dp[i][1]);
            } else if r[j] > r[i] {
                dp[i][0] = dp[i][0].max(dp[j][1] + 1);
                count = count.max(dp[i][0]);
            }
        }
    }

    println!("{}", if count >= 3 { count } else { 0 });
}
