use proconio::input;

fn main() {
    input! {
        a: f64,
        b: f64
    }

    println!("{}", (a / 100.0) * b);
}
