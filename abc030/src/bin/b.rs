use proconio::input;

fn main() {
    input! {
        n: f64,
        m: f64
    }

    let mut a = (30.0 * n + m / 2.0 - 6.0 * m).abs() % 360.0;
    if a > 180.0 {
        a = 360.0 - a;
    }

    println!("{}", a);
}
