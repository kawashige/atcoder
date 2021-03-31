use proconio::input;

fn main() {
    input! {
        a: f64,
        b: f64,
        x: f64
    }

    if x * 2.0 <= a * a * b {
        let r = (a * b * b / (2.0 * x)).atan();
        let theta = r * 180.0 / std::f64::consts::PI;
        println!("{}", theta);
    } else {
        let r = (2.0 * b / a - 2.0 * x / (a * a * a)).atan();
        let theta = r * 180.0 / std::f64::consts::PI;
        println!("{}", theta);
    }
}
