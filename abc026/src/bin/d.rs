use proconio::input;

fn main() {
    input! {
        a: f64,
        b: f64,
        c: f64
    }

    let mut low = 0.0;
    let mut high = 200.0;

    for _ in 0..100 {
        let mid = (low + high) / 2.0;

        if a * mid + b * (c * mid * std::f64::consts::PI).sin() < 100.0 {
            low = mid;
        } else {
            high = mid
        }
    }

    println!("{}", low);
}
