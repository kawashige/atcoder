use proconio::input;

fn main() {
    input! {
        a: f64,
        b: f64,
        h: f64,
        m: f64
    }

    let h_theta = 30.0 * h + m / 2.0;
    let m_theta = 6.0 * m;
    let mut theta = if h_theta > m_theta {
        h_theta - m_theta
    } else {
        m_theta - h_theta
    };
    if theta > 180.0 {
        theta = 360.0 - theta;
    }
    let c =
        (a * a + b * b - 2.0 * a * b * (theta as f64 * std::f64::consts::PI / 180.0).cos()).sqrt();

    println!("{}", c);
}
