use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [f64; n]
    }

    let sum = a.into_iter().map(|i| 1.0 / i).sum::<f64>();
    println!("{}", 1.0 / sum);
}
