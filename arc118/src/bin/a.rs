use proconio::input;

fn main() {
    input! {
        t: u64,
        n: u64
    }

    println!("{}", (100 * n + t - 1) / t + n - 1);
}
