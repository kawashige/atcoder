use proconio::input;

fn main() {
    input! {
        k: usize
    }

    let m = 1_000_000_007;

    if k % 9 != 0 {
        println!("0");
        return;
    }

    let mut dp = vec![0; k + 1];
    dp[0] = 1;

    for i in 1..=k {
        for j in 1..10 {
            if i < j {
                break;
            }
            if dp[i - j] > 0 {
                dp[i] += dp[i - j];
                dp[i] %= m;
            }
        }
    }

    println!("{}", dp[k]);
}
