use proconio::input;

fn main() {
    input! {
        x: usize
    }

    let target = x % 10000;
    let prices = vec![100, 101, 102, 103, 104, 105];
    let mut dp = vec![vec![false; 10000]; prices.len() + 1];
    dp[0][0] = true;

    for i in 0..prices.len() {
        for j in 0..10000 {
            if dp[i][j] {
                dp[i + 1][j] = true;
                for k in ((j + prices[i])..10000).step_by(prices[i]) {
                    dp[i + 1][k] = true;
                }
            }
        }
    }

    if dp[prices.len()][target] {
        println!("1");
    } else {
        println!("0");
    }
}
