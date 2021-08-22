use proconio::input;

fn main() {
    input! {
        n: usize,
        ng: [usize; 3]
    }

    let mut dp = vec![101; n + 1];
    dp[0] = 0;

    for i in 1..=n {
        if ng.contains(&i) {
            continue;
        }

        dp[i] = dp[i].min(dp[i - 1] + 1);
        if i > 1 {
            dp[i] = dp[i].min(dp[i - 2] + 1);
        }
        if i > 2 {
            dp[i] = dp[i].min(dp[i - 3] + 1);
        }
    }

    if dp[n] < 101 {
        println!("YES");
    } else {
        println!("NO");
    }
}
