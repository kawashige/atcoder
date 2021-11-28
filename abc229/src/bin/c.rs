use proconio::input;

fn main() {
    input! {
        n: usize,
        w: u64,
        mut ab: [(u64, u64); n]
    }

    ab.sort_unstable_by(|a, b| b.0.cmp(&a.0));

    let mut used = 0;
    let mut r = 0;
    for (a, b) in ab {
        if b + used <= w {
            used += b;
            r += a * b
        } else {
            r += a * (w - used);
            break;
        }
    }

    println!("{}", r);
}
