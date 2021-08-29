use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut r: [usize; n]
    }

    r.sort_unstable_by(|a, b| b.cmp(&a));

    let mut m = 0.5;
    let mut rate = 0.0;

    for x in r.into_iter().take(k) {
        rate += m * x as f64;
        m *= 0.5;
    }

    println!("{}", rate);
}
