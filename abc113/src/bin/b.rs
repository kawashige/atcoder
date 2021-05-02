use proconio::input;

fn main() {
    input! {
        n: usize,
        t: f64,
        a: f64,
        h: [f64; n]
    }

    let mut min = std::f64::MAX;
    let mut min_i = 0;
    for i in 0..n {
        let d = (a - (t - h[i] * 0.006)).abs();
        if d < min {
            min = d;
            min_i = i;
        }
    }

    println!("{}", min_i + 1)
}
