use proconio::input;

fn main() {
    input! {
        n: u128,
        k: u128
    }

    let mut sum: u128 = 0;
    for i in k..=(n + 1) {
        let max = n * (n + 1) / 2
            - if i == n + 1 {
                0
            } else {
                (n - i) * (n + 1 - i) / 2
            };
        let min = i * (i - 1) / 2;
        sum = (sum + max - min + 1) % 1_000_000_007;
    }
    println!("{}", sum);
}
