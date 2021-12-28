use proconio::input;

fn main() {
    input! {
        n: usize,
        d: usize,
        mut lr: [(usize, usize); n]
    }

    lr.sort_unstable_by_key(|(_, r)| *r);

    let mut i = 0;
    let mut broken = 0;
    let mut count = 0;

    while i < n {
        if broken < lr[i].0 {
            count += 1;
            broken = lr[i].1 + d - 1;
        }
        i += 1;
    }

    println!("{}", count);
}
