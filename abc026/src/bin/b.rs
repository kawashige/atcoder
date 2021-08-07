use proconio::input;

fn main() {
    input! {
        n: usize,
        mut r: [usize; n]
    }

    r.sort_unstable_by(|a, b| b.cmp(&a));

    let mut area = 0;
    for i in 0..n {
        if i % 2 == 0 {
            area += r[i] * r[i];
        } else {
            area -= r[i] * r[i];
        }
    }

    println!("{}", area as f64 * std::f64::consts::PI);
}
