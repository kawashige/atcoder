use proconio::input;

fn main() {
    input! {
        r: usize
    }

    println!("{}", 2.0 * std::f64::consts::PI * r as f64);
}
