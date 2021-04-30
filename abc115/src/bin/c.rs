use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut h: [usize; n]
    }

    h.sort_unstable_by(|a, b| b.cmp(&a));
    let min = (0..=(n - k)).map(|i| h[i] - h[i + k - 1]).min().unwrap();

    println!("{}", min);
}
