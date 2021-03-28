use proconio::input;

fn main() {
    input! {
        n: usize,
        x0: i32,
        y0: i32,
        x2: i32,
        y2: i32
    }

    let theta = ((360.0 / (n as f64)) / 180.0) * std::f64::consts::PI;

    let (c1, c2) = (
        x2 as f64 + 0.5 * (x0 - x2) as f64,
        y2 as f64 + 0.5 * (y0 - y2) as f64,
    );

    let (x1, y1) = (x0 as f64 - c1, y0 as f64 - c2);
    let (x1, y1) = (
        theta.cos() * x1 as f64 - theta.sin() * y1 as f64 + c1,
        theta.sin() * x1 as f64 + theta.cos() * y1 as f64 + c2,
    );

    println!("{} {}", x1, y1);
}
