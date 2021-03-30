use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [(i32, i32); n]
    }

    let mut d = 0.0;
    for i in 0..n {
        for j in (i + 1)..n {
            d += (((p[i].0 - p[j].0).pow(2_u32) + (p[i].1 - p[j].1).pow(2_u32)) as f64).sqrt();
        }
    }
    println!("{}", 2.0 * d / n as f64);
}
