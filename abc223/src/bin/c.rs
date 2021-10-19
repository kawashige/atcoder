use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(f64, f64); n]
    }

    let mut target = ab.iter().map(|(a, b)| a / b).sum::<f64>() / 2.0;
    let mut l = 0.0;

    for i in 0..n {
        let t = ab[i].0 / ab[i].1;
        if target <= t {
            l += ab[i].1 * target;
            println!("{}", l);
            return;
        } else {
            target -= t;
            l += ab[i].0;
        }
    }
}
