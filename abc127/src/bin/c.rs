use proconio::input;

fn main() {
    input! {
        _n: usize,
        m: usize,
        mut lr: [(usize, usize); m]
    }

    lr.sort_unstable();

    let (mut min, mut max) = lr[0];
    for i in 1..m {
        if max < lr[i].0 {
            println!("0");
            return;
        }
        min = lr[i].0;
        max = std::cmp::min(max, lr[i].1);
    }

    println!("{}", max - min + 1);
}
