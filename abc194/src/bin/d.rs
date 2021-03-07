use proconio::input;

fn main() {
    input! {
        n: usize
    }

    let mut e: f64 = 0.0;
    for i in 1..=n {
        e += n as f64 / i as f64;
    }

    println!("{}", e - 1.0);
}
