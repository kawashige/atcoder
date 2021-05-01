use proconio::input;

fn main() {
    input! {
        n: usize,
        d: f64,
        h: f64,
        dh: [(f64, f64); n]
    }

    let mut max = 0.0;
    for (dd, hh) in dh {
        if max < h - d * (h - hh) / (d - dd) {
            max = h - d * (h - hh) / (d - dd);
        }
    }

    println!("{}", max);
}
