use proconio::input;

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
        mut s: [u64; n]
    }

    s.sort_unstable();

    if s[0] == s[n - 1] {
        println!("-1");
        return;
    }

    let sum = s.iter().sum::<u64>();
    let p = b as f64 / (s[n - 1] - s[0]) as f64;
    let q = a as f64 - p * sum as f64 / n as f64;

    println!("{} {}", p, q);
}
