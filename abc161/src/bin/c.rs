use proconio::input;

fn main() {
    input! {
        n: i64,
        k: i64
    }

    let n = std::cmp::min(n, std::cmp::min(n % k, (n % k - k).abs()));
    println!("{}", n)
}
